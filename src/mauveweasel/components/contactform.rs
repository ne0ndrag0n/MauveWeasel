use serde_urlencoded;
use serde_json;

#[derive(Deserialize)]
pub struct CommentValidation {
    pub name_valid: Option< bool >,
    pub comment_valid: Option< bool >
}

pub fn get_validation( query_string: &str ) -> Vec< &str > {
    let validation = match serde_urlencoded::from_str( query_string ) {
        Ok( good ) => good,
        Err( _ ) => CommentValidation{ name_valid: Some( true ), comment_valid: Some( true ) }
    };

    let mut result = Vec::new();
    if validation.name_valid.is_some() && validation.name_valid.unwrap() == false {
        result.push( "Please provide a name." );
    }

    if validation.comment_valid.is_some() && validation.comment_valid.unwrap() == false {
        result.push( "Please provide a comment." );
    }

    result
}
