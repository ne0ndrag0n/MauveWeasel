use bincode;
use comrak::{ ComrakOptions, markdown_to_html };
use uuid::Uuid;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::{ ErrorKind, Write };
use std::io;
use std::time::SystemTime;
use std::cmp::PartialEq;
use mauveweasel::http::Response;
use mauveweasel::options::Config;
use mauveweasel::server::DynamicContentServer;
use mauveweasel::utility;

#[derive(Eq,Serialize,Deserialize)]
struct Document {
    filename: String,
    modified_time: SystemTime,

    category: Option< String >,
    slug: Option< String >,
    pubdate: Option< String >,
    headline: Option< String >,
    brief: Option< String >,
    takeaways: Vec< String >,
    document_text: Option< String >
}

impl PartialEq for Document {
    fn eq( &self, other: &Document ) -> bool {
        self.filename == other.filename &&
        self.modified_time == other.modified_time
    }
}

#[derive(Serialize)]
struct TocData<'a> {
    document: &'a Document,
    uuid: String
}

#[derive(Serialize)]
struct ArticleData<'a> {
    document: &'a Document,
    article: String
}

pub struct Newsgen {
    index: HashMap< String, Document >
}

impl Document {
    pub fn new( filename: &str, modified_time: SystemTime ) -> Document {
        Document {
            filename: filename.to_owned(),
            modified_time: modified_time,
            category: None,
            slug: None,
            pubdate: None,
            headline: None,
            brief: None,
            takeaways: Vec::new(),
            document_text: None
        }
    }

    /**
     * Lazy-evaluate the document this Document object correlates to
     */
    pub fn process( &mut self ) -> io::Result< () > {
        let file = utility::get_file_string( &self.filename )?;
        let mut lines = file.lines();

        for line in &mut lines {
            // Break at first newline-only line
            if line.len() == 0 {
                break;
            }

            // Parse directive text
            let split: Vec< &str > = line.splitn( 2, ": " ).collect();
            match split[ 0 ] {
                "Category" => self.category = Some( split[ 1 ].to_owned() ),
                "Url" => self.slug = Some( split[ 1 ].to_owned() ),
                "Pubdate" => self.pubdate = Some( split[ 1 ].to_owned() ),
                "Headline" => self.headline = Some( split[ 1 ].to_owned() ),
                "Brief" => self.brief = Some( split[ 1 ].to_owned() ),
                "Takeaway" => self.takeaways.push( split[ 1 ].to_owned() ),
                other => println!( "Invalid directive for markdown article: {}", other )
            }
        }

        // Collect the rest of the lines
        let arr: Vec< &str > = lines.collect();

        // That's the rest of the document
        self.document_text = Some( arr.join( "\n" ) );

        Ok( () )
    }

    pub fn generate_and_respond( &mut self, uuid: &str, server: &DynamicContentServer ) -> Response {
        match self.process() {
            Ok( _ ) => {},
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to process markdown document" )
        };

        let product = match server.templates().render(
            "newsgen/article",
            &ArticleData{ document: &self, article: markdown_to_html( self.document_text(), &ComrakOptions::default() ) }
        ) {
            Ok( product ) => product,
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to render document" )
        };

        match File::create( server.config().cache_directory().to_owned() + &format!( "/{}.html", uuid ) ) {
            Ok( mut file ) => match file.write_all( product.as_bytes() ){
                Ok( _ ) => return Response::create( 200, "text/html", &product ),
                Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to create cached article" )
            },
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to create cached article" )
        };
    }

    pub fn filename( &self ) -> &str {
        &self.filename
    }

    pub fn category( &self ) -> &str {
        match &self.category {
            Some( category ) => category,
            None => ""
        }
    }

    pub fn document_text( &self ) -> &str {
        match &self.document_text {
            Some( text ) => text,
            None => ""
        }
    }
}

impl Newsgen {
    pub fn new() -> Newsgen {
        Newsgen {
            index: HashMap::new()
        }
    }

    fn get_filename_to_uuid_index( &self ) -> HashMap< String, String > {
        let mut result: HashMap< String, String > = HashMap::new();

        for( uuid, document ) in &self.index {
            result.insert( document.filename().to_owned(), uuid.to_owned() );
        }

        result
    }

    fn build_index( &mut self, config: &Config ) -> io::Result< () > {
        let original_uuid_index = self.get_filename_to_uuid_index();

        self.index.clear();

        let listing = self.get_dir_list( config )?;

        // Build index HashMap using this directory
        for mut document in listing {
            self.index.insert( match original_uuid_index.get( document.filename() ) {
                Some( existing_uuid ) => existing_uuid.to_owned(),
                None => format!( "{}", Uuid::new_v4() )
            }, document );
        }

        // Save index
        let mut file = File::create( config.cache_directory().to_owned() + "/ngindex.bin" )?;
        match bincode::serialize_into( &mut file, &self.index ) {
            Ok( _ ) => Ok( () ),
            Err( err ) => return Err( io::Error::new( io::ErrorKind::Other, format!( "{}", err ) ) )
        }
    }

    fn get_dir_list( &self, config: &Config ) -> io::Result< Vec< Document > > {
        let mut result = vec![];
        let directory = fs::read_dir( config.newsgen_directory() )?;

        for entry in directory {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                result.push( Document::new( &entry.path().to_string_lossy(), metadata.modified()? ) );
            }
        }

        Ok( result )
    }

    fn document_equivalent( &self, document: &Document ) -> bool {
        for ( ref _uuid, ref needle ) in self.index.iter() {
            if document == *needle {
                return true
            }
        }

        false
    }

    fn get_sorted_categories_list( &self ) -> HashMap< String, Vec< TocData > > {
        let mut result: HashMap< String, Vec< TocData > > = HashMap::new();

        for ( uuid, document ) in &self.index {
            let mut vec = result.entry( document.category().to_owned() ).or_insert( Vec::new() );

            vec.push( TocData { document: &document, uuid: uuid.to_owned() } );
        }

        result
    }

    fn rebuild_toc( &mut self, server: &DynamicContentServer ) -> Response {
        println!( "rebuilding toc and generating articles" );

        match self.build_index( server.config() ) {
            Ok( _ ) => {

                // Load all documents and parse headers + content
                for ( uuid, document ) in &mut self.index {
                    let response = document.generate_and_respond( &uuid, server );
                    if response.code() == 500 {
                        return response
                    }
                }

                // Generate TOC
                let result = match server.templates().render( "newsgen/toc", &self.get_sorted_categories_list() ) {
                    Ok( result ) => result,
                    Err( msg ) => {
                        println!( "handlebars: {}", msg );
                        return Response::create( 500, "text/plain", "Internal server error: Failed to render document" )
                    }
                };

                // Save TOC to file
                match File::create( server.config().cache_directory().to_owned() + "/toc.html" ) {
                    Ok( mut file ) => match file.write_all( result.as_bytes() ) {
                        Ok( _ ) => return Response::create( 200, "text/html", &result ),
                        Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to write toc.html" )
                    },
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to create toc.html" )
                }

            },
            Err( _ ) => Response::create( 500, "text/plain", "Internal server error: Failed to create ngindex.bin" )
        }
    }

    pub fn respond_individual( uuid: &str, server: &DynamicContentServer ) -> Response {
        // First thing - open file. If the file can't be opened, return 404
        let file = match File::open( server.config().cache_directory().to_owned() + "/ngindex.bin" ) {
            Ok( file ) => file,
            Err( error ) => match error.kind() {
                ErrorKind::NotFound => return Response::create( 404, "text/plain", "Not found" ),
                _ => return Response::create( 500, "text/plain", "Internal server error: Could not open ngindex.bin" )
            }
        };

        let mut newsgen = Newsgen::new();

        // Load index
        newsgen.index = match bincode::deserialize_from( file ) {
            Ok( product ) => product,
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to read ngindex.bin" )
        };

        // If item isn't in index, return 404
        let filename = match newsgen.index.get( uuid ) {
            Some( document ) => document.filename().to_owned(),
            None => return Response::create( 404, "text/plain", "Not found" )
        };

        let path = Path::new( &filename );
        if !path.exists() {
            // If the file behind the document doesn't exist, delete its entry and return 404
            newsgen.index.remove( uuid );

            // Save the new state of newsgen.index and write 404
            match File::create( server.config().cache_directory().to_owned() + "/ngindex.bin" ) {
                Ok( mut file ) => match bincode::serialize_into( &mut file, &newsgen.index ) {
                    Ok( _ ) => return Response::create( 404, "text/plain", "Not found" ),
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to write ngindex.bin" )
                },
                Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to create ngindex.bin" )
            };
        }

        // The file exists but its mtime may have been updated
        let most_current_document = Document::new(
            &filename,
            match path.metadata() {
                Ok( metadata ) => match metadata.modified() {
                    Ok( mtime ) => mtime,
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Could not read mtime for file" )
                },
                Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Could not read mtime for file" )
            }
        );

        // If path mtime is disparate from recorded mtime, rewrite the file
        if *newsgen.index.get( uuid ).expect( "This should never happen" ) != most_current_document {
            println!( "This article needs to be updated" );

            // Reinsert item
            newsgen.index.insert( uuid.to_owned(), most_current_document );

            // Update ngindex.bin
            match File::create( server.config().cache_directory().to_owned() + "/ngindex.bin" ) {
                Ok( mut file ) => match bincode::serialize_into( &mut file, &newsgen.index ) {
                    Ok( _ ) => {},
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to write ngindex.bin" )
                },
                Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to create ngindex.bin" )
            };

            // Regenerate and serve document
            let document = newsgen.index.get_mut( uuid ).expect( "This should never happen" );
            return document.generate_and_respond( uuid, server );
        }

        // If path mtime and document are equivalent, but there was never a file generated, generate the file
        match utility::get_file_string( &( server.config().cache_directory().to_owned() + &format!( "/{}.html", uuid ) ) ) {
            Ok( text ) => Response::create( 200, "text/html", &text ),
            Err( error ) => match error.kind() {
                ErrorKind::NotFound => {
                    println!( "Generating new html cache from unchanged mtime" );
                    return newsgen.index.get_mut( uuid ).expect( "This should never happen" ).generate_and_respond( uuid, server )
                },
                _ => return Response::create( 500, "text/plain", "Internal server error: Failed to open generated cache file" )
            }
        }
    }

    pub fn respond( server: &DynamicContentServer ) -> Response {
        let mut newsgen = Newsgen::new();

        // Open cache/ngindex.bin
        match File::open( server.config().cache_directory().to_owned() + "/ngindex.bin" ) {
            Ok( file ) => {
                newsgen.index = match bincode::deserialize_from( file ) {
                    Ok( product ) => product,
                    Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to read ngindex.bin" )
                };
            },
            Err( error ) => match error.kind() {
                ErrorKind::NotFound => return newsgen.rebuild_toc( server ),
                _ => return Response::create( 500, "text/plain", "Internal server error: Failed to create ngindex.bin" )
            }
        };

        // newsgen.index should be either created or loaded after this point
        // Get directory listing
        let listing = match newsgen.get_dir_list( server.config() ) {
            Ok( listing ) => listing,
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to get directory listing for newsgen" )
        };

        // If directory listing count is different from hashmap count, then we know we need to regenerate the index and html together
        if newsgen.index.len() != listing.len() {
            println!( "newsgen.index {} listing {}", newsgen.index.len(), listing.len() );
            return newsgen.rebuild_toc( server )
        }

        // The numbers of items are equivalent, so verify items in this listing are equivalent to their items in the hashmap
        for needle in listing {
            if !newsgen.document_equivalent( &needle ) {
                // Stop everything and rebuild toc
                println!( "Document was not equivalent" );
                return newsgen.rebuild_toc( server )
            }
        }

        // If you got here then rebuild was not required - open up toc.html and serve it
        match utility::get_file_string( &( server.config().cache_directory().to_owned() + "/toc.html" ) ) {
            Ok( text ) => return Response::create( 200, "text/html", &text ),
            Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to open <cache directory>/toc.html" )
        }
    }
}
