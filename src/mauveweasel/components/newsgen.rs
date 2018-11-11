use bincode;
use uuid::Uuid;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::{ ErrorKind, Write };
use std::io;
use std::time::SystemTime;
use std::cmp::PartialEq;
use handlebars::Handlebars;
use mauveweasel::http::{ Request,Response };
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

    fn build_index( &mut self, config: &Config ) -> io::Result< () > {
        let listing = self.get_dir_list( config )?;

        // Build index HashMap using this directory
        for document in listing {
            self.index.insert( format!( "{}", Uuid::new_v4() ), document );
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

    fn rebuild_toc( &mut self, config: &Config, templates: &Handlebars ) -> Response {
        match self.build_index( config ) {
            Ok( _ ) => {

                // Load all documents and parse headers + content
                for ( _uuid, document ) in &mut self.index {
                    match document.process() {
                        Ok( _ ) => {},
                        Err( _ ) => return Response::create( 500, "text/plain", "Internal server error: Failed to process document" )
                    };
                }

                // Generate TOC
                let result = match templates.render( "newsgen/toc", &self.index ) {
                    Ok( result ) => result,
                    Err( msg ) => {
                        println!( "handlebars: {}", msg );
                        return Response::create( 500, "text/plain", "Internal server error: Failed to render document" )
                    }
                };

                // Save TOC to file
                match File::create( config.cache_directory().to_owned() + "/toc.html" ) {
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

    pub fn respond( request: Request, server: &DynamicContentServer ) -> Response {
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
                ErrorKind::NotFound => return newsgen.rebuild_toc( server.config(), server.templates() ),
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
            return newsgen.rebuild_toc( server.config(), server.templates() )
        }

        // The numbers of items are equivalent, so verify items in this listing are equivalent to their items in the hashmap

        Response::create( 501, "text/plain", "Not implemented" )
    }
}
