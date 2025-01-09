use bitcoin::util::address;
use {super::*, clap::ValueEnum};

#[derive(Default, ValueEnum, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Chain {
  #[default]
  #[clap(alias("main"))]
  Mainnet,
  #[clap(alias("test"))]
  Testnet,
  Signet,
  Regtest,
}

impl Chain {
  pub(crate) fn network(self) -> Network {
    match self {
      Self::Mainnet => Network::Bitcoin,
      Self::Testnet => Network::Testnet,
      Self::Signet => Network::Signet,
      Self::Regtest => Network::Regtest,
    }
  }

  pub(crate) fn default_rpc_port(self) -> u16 {
    match self {
      Self::Mainnet => 32552,
      Self::Regtest => 18332,
      Self::Signet => 38332,
      Self::Testnet => 44873,
    }
  }

  pub(crate) fn inscription_content_size_limit(self) -> Option<usize> {
    match self {
      Self::Mainnet | Self::Regtest => None,
      Self::Testnet | Self::Signet => None,
    }
  }

  pub(crate) fn first_inscription_height(self) -> u32 {
    match self {
      Self::Mainnet => 0,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 0,
    }
  }

  pub(crate) fn first_dune_height(self) -> u32 {
    match self {
      Self::Mainnet => 0,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 0,
    }
  }

  pub(crate) fn genesis_block(self) -> Block {
    let genesis_hex: &str = match self {
      Self::Mainnet => "0100000000000000000000000000000000000000000000000000000000000000000000006159e26e6bcfbb78b4308944c811ef090579427d0fc0a3788a70e241974546d677e15167f0ff0f1e089b00000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2604ffff001d01041e30352f31322f32303234202d205468697320697320466c6f70636f696e2effffffff010058850c02000000434104a5c56a22caf4b1c8917f8a261a83efe5885526d05ad0cb7e9fc7d000d04085e225e578d8f899d3d9811f39f2e7c2c7322c8a070fd757e0c935dd6c35fc8c66ccac00000000",
      Self::Regtest => "0100000000000000000000000000000000000000000000000000000000000000000000006159e26e6bcfbb78b4308944c811ef090579427d0fc0a3788a70e241974546d677e15167f0ff0f1e089b00000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2604ffff001d01041e30352f31322f32303234202d205468697320697320466c6f70636f696e2effffffff010058850c02000000434104a5c56a22caf4b1c8917f8a261a83efe5885526d05ad0cb7e9fc7d000d04085e225e578d8f899d3d9811f39f2e7c2c7322c8a070fd757e0c935dd6c35fc8c66ccac00000000",
      Self::Signet => "010000000000000000000000000000000000000000000000000000000000000000000000696ad20e2dd4365c7459b4a4a5af743d5e92c6da3229e6532cd605f6533f2a5bb9a7f052f0ff0f1ef7390f000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff1004ffff001d0104084e696e746f6e646fffffffff010058850c020000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
      Self::Testnet => "0100000000000000000000000000000000000000000000000000000000000000000000006159e26e6bcfbb78b4308944c811ef090579427d0fc0a3788a70e241974546d677e15167f0ff0f1e089b00000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2604ffff001d01041e30352f31322f32303234202d205468697320697320466c6f70636f696e2effffffff010058850c02000000434104a5c56a22caf4b1c8917f8a261a83efe5885526d05ad0cb7e9fc7d000d04085e225e578d8f899d3d9811f39f2e7c2c7322c8a070fd757e0c935dd6c35fc8c66ccac00000000",
    };
    let genesis_buf: Vec<u8> = hex::decode(genesis_hex).unwrap();
    bitcoin::consensus::deserialize(&genesis_buf).unwrap()
  }

  pub(crate) fn address_from_script(
    self,
    script: &Script,
  ) -> Result<Address, address::Error> {
    Address::from_script(script, self.network())
  }

  pub(crate) fn join_with_data_dir(self, data_dir: &Path) -> PathBuf {
    match self {
      Self::Mainnet => data_dir.to_owned(),
      Self::Testnet => data_dir.join("testnet3"),
      Self::Signet => data_dir.join("signet"),
      Self::Regtest => data_dir.join("regtest"),
    }
  }
}

impl Display for Chain {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Mainnet => "mainnet",
        Self::Regtest => "regtest",
        Self::Signet => "signet",
        Self::Testnet => "testnet",
      }
    )
  }
}
