use ::aes::Aes256;
use ::block_modes::block_padding::Iso7816;

use ::block_modes::Cbc;

use ::twofish::Twofish;

type Aes256Cbc = Cbc<Aes256, Iso7816>;
type TwofishCbc = Cbc<Twofish, Iso7816>;

pub mod encrypt;

pub mod decrypt;
