use std::collections::HashMap;
use mauveweasel::options::Config;
use chrono::{ Duration, DateTime, Utc };
use std::io;

pub trait Cookie {
   fn name( &self ) -> &str;
   fn value( &self ) -> &str;
   fn max_age( &self ) -> Option< Duration >;
   fn get_expiry( &self ) -> Option< DateTime< Utc > >;
   fn save( &self, config: &Config ) -> io::Result< () >;
}

pub fn parse( cookie_string: &str ) -> HashMap< String, String > {
    let mut result = HashMap::new();

    let cookie_pair_tokens = cookie_string.trim().split( ';' );
    for cookie_pair_token in cookie_pair_tokens {
        let pair: Vec< &str > = cookie_pair_token.trim().split( '=' ).collect();
        if pair.len() == 2 {
            result.insert( pair[ 0 ].trim().to_string(), pair[ 1 ].trim().to_string() );
        }
    }

    result
}
