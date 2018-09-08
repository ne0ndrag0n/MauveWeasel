pub struct Comment {
    honeypot: String,
    comment: String
}

static HONEYPOT: &'static str = "support@ne0ndrag0n.com";

impl Comment {
    pub fn validate( &self ) -> bool {
        self.honeypot == HONEYPOT
    }

    pub fn honeypot( &self ) -> &String {
        &self.honeypot
    }

    pub fn comment( &self ) -> &String {
        &self.comment
    }
}
