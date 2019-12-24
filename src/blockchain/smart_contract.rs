
use openssl::rsa::Rsa;
use openssl::pkey::{PKey, Private};
use std::str;
use openssl::x509::{X509, X509Name};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::sign::{Signer, Verifier};
use std::fs::File;
use std::io::{Write, Read};

pub struct SmartContract {
    private_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
    pkey: PKey<Private>,
}

impl SmartContract {

    ///Generate a new pair of private and public key
    pub fn generate_keys() -> SmartContract {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let mut name = X509Name::builder().unwrap();
        name.append_entry_by_nid(Nid::COMMONNAME, "foobar.com").unwrap();
        let name = name.build();
        let mut builder = X509::builder().unwrap();
        builder.set_version(2).unwrap();
        builder.set_subject_name(&name).unwrap();
        builder.set_issuer_name(&name).unwrap();
        builder.set_pubkey(&pkey).unwrap();
        builder.sign(&pkey, MessageDigest::sha256()).unwrap();
        let certificate: X509 = builder.build();
        let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
        println!("{}", str::from_utf8(pub_key.as_slice()).unwrap());
        let pri_key: Vec<u8> = pkey.private_key_to_pem_pkcs8().unwrap();
        println!("{}", str::from_utf8(pri_key.as_slice()).unwrap());
        let pem: Vec<u8> = certificate.to_pem().unwrap();
        println!("{}", str::from_utf8(pem.as_slice()).unwrap());
        SmartContract{
                private_key: Some(pri_key),
                public_key: Some(pub_key),
                pkey
            }
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8>{
        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey).unwrap();
        signer.update(data).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        signature
    }

    pub fn verify_signature(&self, signature: &Vec<u8>, data: &[u8]) -> bool {
        let mut verifier = Verifier::new(MessageDigest::sha256(), &self.pkey).unwrap();
        verifier.update(data).unwrap();
        verifier.verify(&signature).unwrap()
    }


    pub fn load_keys(&mut self){
        let pub_key_name = "pub.key";
        let private_key_name = "private.key";

        let mut data = Vec::new();
        let mut f = File::open(pub_key_name).expect("Unable to open file");
        f.read_to_end(&mut data).expect("Unable to read data");

        self.public_key = Some(data);

        let mut data2 = Vec::new();
        let mut f2 = File::open(private_key_name).expect("Unable to open file");
        f2.read_to_end(&mut data2).expect("Unable to read data");

        self.private_key = Some(data2);

    }

    pub fn save_keys(&self){
        let pub_key_name = "pub.key";
        let private_key_name = "private.key";

        let mut f = File::create(pub_key_name).expect("Unable to create file");
        f.write_all(self.public_key.as_ref().unwrap().as_slice()).expect("Unable to write data");

        let mut f2 = File::create(private_key_name).expect("Unable to create file");
        f2.write_all(self.private_key.as_ref().unwrap().as_slice()).expect("Unable to write data");

    }

}

#[cfg(test)]
mod tests {
    use crate::blockchain::smart_contract::SmartContract;

    #[test]
    pub fn test_signature(){
        let contract = SmartContract::generate_keys();

        let signature = contract.sign(b"hello, world");
        assert_eq!(contract.verify_signature(&signature, b"hello, world"), true)
    }

    #[test]
    pub fn test_saving_keys(){
        let mut contract = SmartContract::generate_keys();

        contract.save_keys();
        contract.load_keys();

    }

}