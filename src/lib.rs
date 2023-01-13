use librypt_entropy::EntropySource;
use librypt_kap::KeyAgreementProtocol;

/// A Diffie-Hellman shared key.
pub struct DhSharedKey<T> {
    value: T,
}

/// A Diffie-Hellman shared secret.
pub struct DhSharedSecret<T: AsRef<[u8]>> {
    value: T,
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for DhSharedSecret<T> {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}

/// Diffie-Hellman implementation.
pub struct DiffieHellman<T: AsRef<[u8]>> {
    secret: T,
}

impl<T: AsRef<[u8]>> KeyAgreementProtocol for DiffieHellman<T> {
    type SharedKey = DhSharedKey<T>;
    type SharedSecret = DhSharedSecret<T>;

    fn shared_key(&self) -> Self::SharedKey {
        todo!()
    }

    fn compute_shared_secret(&self, shared_key: &Self::SharedKey) -> Self::SharedSecret {
        todo!()
    }
}
