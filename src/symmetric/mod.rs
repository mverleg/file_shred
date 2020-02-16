use ::aes::Aes256;
use ::block_modes::block_padding::Iso7816;
use ::block_modes::BlockMode;
use ::block_modes::Cbc;
use ::secstr::SecVec;
use ::twofish::Twofish;

use crate::header::SymmetricEncryptionAlg;
use crate::key::key::StretchKey;
use crate::key::Salt;
use crate::symmetric::shared::endec_aes256;
use crate::util::FedResult;

type Aes256Cbc = Cbc<Aes256, Iso7816>;
type TwofishCbc = Cbc<Twofish, Iso7816>;

pub mod encrypt;

pub mod decrypt;

mod shared;
