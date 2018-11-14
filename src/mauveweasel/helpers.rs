use handlebars::*;

pub fn ifval<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    ctx: &Context,
    rc: &mut RenderContext<'reg>,
    out: &mut Output,
) -> HelperResult {
    let needle = h.param( 0 ).and_then( |v| v.value().as_str() ).unwrap_or( "" );
    let value = h.param( 1 ).and_then( |v| v.value().as_str() ).unwrap_or( "" );

    if needle == value {
        match h.template() {
            Some( template ) => match template.render( r, ctx, rc, out ) {
                Ok( _ ) => {},
                Err( _ ) => return Err( RenderError::new( "Couldn't render template!" ) )
            },
            None => return Err( RenderError::new( "Couldn't unwrap template!" ) )
        }
    } else {
        match h.inverse() {
            Some( template ) => match template.render( r, ctx, rc, out ) {
                Ok( _ ) => {},
                Err( _ ) => return Err( RenderError::new( "Couldn't render inverse!" ) )
            },
            None => {}
        }
    }

     Ok( () )
}
