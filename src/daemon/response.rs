#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedUrlResponse {
    pub data: SignedUrlData,
}

impl SignedUrlResponse {

    pub fn new() -> SignedUrlResponse {
        Default::default()
    }

    pub fn set_attributes(&mut self, val: SignedUrlAttributes) {
        self.data.set_attributes(val);
    }

}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedUrlData {
    pub attributes: SignedUrlAttributes,
}

impl SignedUrlData {

    pub fn new() -> SignedUrlData {
        Default::default()
    }

    pub fn set_attributes(&mut self, val: SignedUrlAttributes) {
        self.attributes = val;
    }


}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedUrlAttributes {
    pub url: String,
    pub method: String,
    pub ttl: i64,
    pub engine: String,
    pub request: String,
}

impl SignedUrlAttributes {

    pub fn new() -> SignedUrlAttributes {
        Default::default()
    }

    pub fn set_url(&mut self, val: String) {
        self.url = val;
    }

    pub fn set_method(&mut self, val: String) {
        self.method = val;
    }

    pub fn set_ttl(&mut self, val: i64) {
        self.ttl = val;
    }

    pub fn set_engine(&mut self, val: String) {
        self.engine = val;
    }

    pub fn set_request(&mut self, val: String) {
        self.request = val;
    }

}
