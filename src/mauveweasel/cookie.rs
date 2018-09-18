pub trait Cookie {
   fn name( &self ) -> &str;
   fn value( &self ) -> &str;
   fn save( &self );
}