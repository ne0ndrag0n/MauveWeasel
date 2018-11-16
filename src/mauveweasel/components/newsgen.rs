use bincode;
use comrak::{ ComrakOptions, markdown_to_html };
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io;
use std::time::SystemTime;
use std::cmp::PartialEq;
use mauveweasel::http::Response;
use mauveweasel::options::Config;
use mauveweasel::server::DynamicContentServer;
use mauveweasel::utility;
use mauveweasel::tools::uuid_keystore;
use lru_cache::LruCache;

#[derive(Serialize,Deserialize)]
pub struct Document {
    uuid: String,
    category: String,
    slug: String,
    pubdate: String,
    headline: String,
    brief: String,
    takeaways: Vec< String >,
    document_text: String
}

#[derive(Eq,Serialize,Deserialize,Hash)]
struct FileIndex {
    filename: String,
    modified_time: SystemTime
}

impl FileIndex {
    pub fn new( filename: String, modified_time: SystemTime ) -> FileIndex {
        FileIndex{ filename, modified_time }
    }

    pub fn filename( &self ) -> &str {
        &self.filename
    }
}

#[derive(Eq,Hash,Serialize,Deserialize)]
enum CacheKey {
    Document( FileIndex ),
    Aggregate( Vec< FileIndex > )
}

impl CacheKey {
    pub fn unwrap_document( &self ) -> &FileIndex {
        match self {
            CacheKey::Document( result ) => &result,
            _ => panic!( "Incorrect type" )
        }
    }

    pub fn unwrap_aggregate( &self ) -> &Vec< FileIndex > {
        match self {
            CacheKey::Aggregate( result ) => &result,
            _ => panic!( "Incorrect type" )
        }
    }
}

impl PartialEq for CacheKey {
    fn eq( &self, other: &CacheKey ) -> bool {
        match self {
            CacheKey::Document( index ) => match other {
                CacheKey::Document( other_index ) => index == other_index,
                _ => false
            },
            CacheKey::Aggregate( list ) => match other {
                CacheKey::Aggregate( other_list ) => {
                    if list.len() != other_list.len() {
                        return false;
                    }

                    for item in list {
                        let mut found_once = false;
                        for other_item in other_list {
                            if item == other_item { found_once = true; break; }
                        }
                        if !found_once {
                            return false;
                        }
                    }

                    true
                },
                _ => false
            }
        }
    }
}

impl PartialEq for FileIndex {
    fn eq( &self, other: &FileIndex ) -> bool {
        self.filename == other.filename &&
        self.modified_time == other.modified_time
    }
}

pub struct Newsgen {
    cache: LruCache< CacheKey, String >
}

impl Document {
    pub fn new() -> Document {
        Document {
            uuid: String::new(),
            category: String::new(),
            slug: String::new(),
            pubdate: String::new(),
            headline: String::new(),
            brief: String::new(),
            takeaways: Vec::new(),
            document_text: String::new()
        }
    }

    pub fn from_file( path: &str, config: &Config ) -> io::Result< Document > {
        let mut result = Document::new();
        result.uuid = match uuid_keystore::get_or_associate_uuid( path, config ) {
            Ok( uuid ) => uuid,
            Err( _ ) => return Err( io::Error::new( io::ErrorKind::Other, "" ) )
        };

        let file = utility::get_file_string( path )?;
        let mut lines = file.lines();

        for line in &mut lines {
            // Break at first newline-only line
            if line.len() == 0 {
                break;
            }

            // Parse directive text
            let split: Vec< &str > = line.splitn( 2, ": " ).collect();
            match split[ 0 ] {
                "Category" => result.category = split[ 1 ].to_owned(),
                "Url" => result.slug = split[ 1 ].to_owned(),
                "Pubdate" => result.pubdate = split[ 1 ].to_owned(),
                "Headline" => result.headline = split[ 1 ].to_owned(),
                "Brief" => result.brief = split[ 1 ].to_owned(),
                "Takeaway" => result.takeaways.push( split[ 1 ].to_owned() ),
                other => println!( "Invalid directive for markdown article: {}", other )
            }
        }

        // Collect the rest of the lines
        let arr: Vec< &str > = lines.collect();

        // That's the rest of the document
        result.document_text = arr.join( "\n" );

        Ok( result )
    }

    pub fn generate( &mut self, server: &DynamicContentServer ) -> Result< String, &'static str > {
        // Use comrak to convert document_text to article_text
        self.document_text = markdown_to_html( &self.document_text, &ComrakOptions::default() );

        // Use handlebars to convert Document to HTML
        match server.templates().render( "newsgen/article", self ) {
            Ok( product ) => return Ok( product ),
            Err( _ ) => return Err( "Failed to render document" )
        };
    }

    pub fn category( &self ) -> &str {
        &self.category
    }

    pub fn document_text( &self ) -> &str {
        &self.document_text
    }
}

impl Newsgen {
    pub fn new( cache_size: usize ) -> Newsgen {
        Newsgen {
            cache: LruCache::new( cache_size )
        }
    }

    fn get_dir_list( config: &Config ) -> io::Result< Vec< FileIndex > > {
        let mut result = vec![];
        let directory = fs::read_dir( config.newsgen_directory() )?;

        for entry in directory {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                result.push( FileIndex::new( entry.path().to_string_lossy().to_string(), metadata.modified()? ) );
            }
        }

        Ok( result )
    }

    fn load_cache_from_file( &mut self, config: &Config ) -> io::Result< () > {
        let file = File::open( config.cache_directory().to_owned() + "/ngcache.bin" )?;
        let vec: Vec< ( CacheKey, String ) > = match bincode::deserialize_from( file ) {
            Ok( product ) => product,
            Err( _ ) => return Ok( () )
        };

        for ( cachekey, string ) in vec {
            self.cache.insert( cachekey, string );
        }

        Ok( () )
    }

    fn save_cache_to_file( &mut self, config: &Config ) -> io::Result< () > {
        let file = File::create( config.cache_directory().to_owned() + "/ngcache.bin" )?;
        let mut vec: Vec< ( CacheKey, String ) > = Vec::new();

        while let Some( ( cachekey, value ) ) = self.cache.remove_lru() {
            vec.push( ( cachekey, value ) );
        }

        match bincode::serialize_into( &file, &vec ) {
            Ok( _ ) => Ok( () ),
            Err( _ ) => Err( io::Error::new( io::ErrorKind::Other, "" ) )
        }
    }

    fn sort_toc_data<'a>( &self, data: &'a Vec< Document > ) -> HashMap< String, Vec< &'a Document > > {
        let mut result: HashMap< String, Vec< &Document > > = HashMap::new();

        for document in data {
            let mut vec = result.entry( document.category().to_owned() ).or_insert( Vec::new() );

            vec.push( &document );
        }

        result
    }

    fn generate_toc( &self, server: &DynamicContentServer, dir_list: &Vec< FileIndex > ) -> Result< String, &'static str > {
        println!( "rebuilding toc" );

        let mut documents: Vec< Document > = Vec::new();
        for dir in dir_list {
            documents.push( match Document::from_file( dir.filename(), server.config() ) {
                Ok( result ) => result,
                Err( _ ) => return Err( "Failed to open document" )
            } );
        }

        let result = match server.templates().render( "newsgen/toc", &self.sort_toc_data( &documents ) ) {
            Ok( result ) => result,
            Err( msg ) => { println!( "{}", msg ); return Err( "Failed to render document" ) }
        };

        Ok( result )
    }

    pub fn respond_individual( uuid: &str, server: &DynamicContentServer ) -> Response {
        let full = match uuid_keystore::get_value( uuid, server.config() ) {
            Ok( result ) => match result {
                Some( filename ) => filename,
                None => return Response::create( 404, "text/plain", "Not found" )
            },
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Could not open uuid keystore" )
        };

        let path = Path::new( &full );
        if !path.exists() || !path.is_file() {
            return Response::create( 404, "text/plain", "Not found" );
        }

        // Check and see if it's in the cache first!
        let mut newsgen = Newsgen::new( server.config().newsgen_lru_cache_size() );
        match newsgen.load_cache_from_file( server.config() ) {
            Ok( _ ) => {},
            Err( error ) => match error.kind() {
                ErrorKind::NotFound => match newsgen.save_cache_to_file( server.config() ) {
                    Ok( _ ) => (),
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Could not create cache file" )
                },
                _ => return Response::create( 500, "text/plain", "Internal server error: Could not open cache file" )
            }
        };

        let key = CacheKey::Document(
            FileIndex::new( full.to_owned(), match path.metadata() {
                Ok( metadata ) => match metadata.modified() {
                    Ok( modified ) => modified,
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Cannot read path metadata" )
                },
                Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Cannot read path metadata" )
            } )
        );

        match newsgen.cache.get_mut( &key ) {
            Some( result ) => return Response::create( 200, "text/html", result ),
            None => { /* Must regenerate */ }
        };

        println!( "regenerating document" );

        let mut document = match Document::from_file( &full, server.config() ) {
            Ok( document ) => document,
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Cannot open markdown document" )
        };

        match document.generate( server ) {
            Ok( result ) => {
                newsgen.cache.insert( key, result.to_owned() );

                match newsgen.save_cache_to_file( server.config() ) {
                    Ok( _ ) => Response::create( 200, "text/html", &result ),
                    Err( _ ) => Response::create( 500, "text/plain", "Internal server error: Failed to save cache file" )
                }
            },
            Err( error ) => Response::create( 500, "text/plain", &format!( "Internal server error: {}", error ) )
        }
    }

    pub fn respond( server: &DynamicContentServer ) -> Response {
        let dir_list = match Newsgen::get_dir_list( server.config() ) {
            Ok( list ) => list,
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Unable to get directory list" )
        };

        let mut newsgen = Newsgen::new( server.config().newsgen_lru_cache_size() );
        match newsgen.load_cache_from_file( server.config() ) {
            Ok( _ ) => {},
            Err( error ) => match error.kind() {
                ErrorKind::NotFound => match newsgen.save_cache_to_file( server.config() ) {
                    Ok( _ ) => (),
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Could not create cache file" )
                },
                _ => return Response::create( 500, "text/plain", "Internal server error: Could not open cache file" )
            }
        };

        let aggregate = CacheKey::Aggregate( dir_list );
        match newsgen.cache.get_mut( &aggregate ) {
            Some( cached_rendering ) => return Response::create( 200, "text/html", cached_rendering ),
            None => { /* Must Regenerate */ }
        };

        // Insert, generate, and store
        match newsgen.generate_toc( server, aggregate.unwrap_aggregate() ) {
            Ok( rendered_template ) => {
                newsgen.cache.insert( aggregate, rendered_template.to_owned() );

                match newsgen.save_cache_to_file( server.config() ) {
                    Ok( _ ) => Response::create( 200, "text/html", &rendered_template ),
                    Err( _ ) => Response::create( 500, "text/plain", "Internal server error: Failed to save cache file" )
                }
            },
            Err( message ) => Response::create( 500, "text/plain", &format!( "Internal server error: {}", message ) )
        }
    }
}
