use crate::{Bytes, Error, Precompile, PrecompileError, PrecompileResult, PrecompileWithAddress};
use parity_bytes::BytesRef;
use tendermint::lite::light_client;

pub const TENDERMINT_HEADER_VALIDATION: PrecompileWithAddress = PrecompileWithAddress(
    crate::u64_to_address(100),
    Precompile::Standard(crate::tendermint::tendermint_header_validation_run),
);

fn tendermint_header_validation_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    const TENDERMINT_HEADER_VALIDATION_BASE: u64 = 3_000;

    if TENDERMINT_HEADER_VALIDATION_BASE > gas_limit {
        return Err(Error::OutOfGas);
    }

    let mut output = vec![0u8, 0, 0];
    let mut bytes = BytesRef::Flexible(&mut output);
    let res = light_client::TmHeaderVerifier::execute(input.as_ref(), &mut bytes);
    match res {
        Ok(()) => Ok((TENDERMINT_HEADER_VALIDATION_BASE, Bytes::from(output))),
        Err(str) => Err(PrecompileError::Other(String::from(str))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm_primitives::hex;

    #[test]
    fn test_tendermint_header_validation_run() {
        let input = hex::decode("0000000000000000000000000000000000000000000000000000000000001325000000000000000000000000000000000000000000000000000000000000022042696e616e63652d436861696e2d4e696c6500000000000000000000000000000000000003fc05e2b7029751d2a6581efc2f79712ec44d8b4981850325a7feadaa58ef4ddaa18a9380d9ab0fc10d18ca0e0832d5f4c063c5489ec1443dfb738252d038a82131b27ae17cbe9c20cdcfdf876b3b12978d3264a007fcaaa71c4cdb701d9ebc0323f44f000000174876e800184e7b103d34c41003f9b864d5f8c1adda9bd0436b253bb3c844bc739c1e77c9000000174876e8004d420aea843e92a0cfe69d89696dff6827769f9cb52a249af537ce89bf2a4b74000000174876e800bd03de9f8ab29e2800094e153fac6f696cfa512536c9c2f804dcb2c2c4e4aed6000000174876e8008f4a74a07351895ddf373057b98fae6dfaf2cd21f37a063e19601078fe470d53000000174876e8004a5d4753eb79f92e80efe22df7aca4f666a4f44bf81c536c4a09d4b9c5b654b5000000174876e800c80e9abef7ff439c10c68fe8f1303deddfc527718c3b37d8ba6807446e3c827a000000174876e8009142afcc691b7cc05d26c7b0be0c8b46418294171730e079f384fde2fa50bafc000000174876e80049b288e4ebbb3a281c2d546fc30253d5baf08993b6e5d295fb787a5b314a298e000000174876e80004224339688f012e649de48e241880092eaa8f6aa0f4f14bfcf9e0c76917c0b6000000174876e8004034b37ceda8a0bf13b1abaeee7a8f9383542099a554d219b93d0ce69e3970e8000000174876e800e3210a92130abb020a02080a121242696e616e63652d436861696e2d4e696c6518e38bf01f220c08e191aef20510f5f4e4c70230dae0c7173a480a20102b54820dd8fb5bc2c4e875ee573fa294d9b7b7ceb362aa8fd21b33dee41b1c12240801122082f341511f3e6b89d6177fd31f8a106013ba09d6e12ef40a7dec885d81b687634220b1b77e6977e0cd0177e3102a78833c9e152aa646ed4fb5a77e8af58c9867eec0522080d9ab0fc10d18ca0e0832d5f4c063c5489ec1443dfb738252d038a82131b27a5a2080d9ab0fc10d18ca0e0832d5f4c063c5489ec1443dfb738252d038a82131b27a6220294d8fbd0b94b767a7eba9840f299a3586da7fe6b5dead3b7eecba193c400f936a20a3e248bc209955054d880e4d89ff3c0419c0cd77681f4b4c6649ead5545054b982011462633d9db7ed78e951f79913fdc8231aa77ec12b12d1100a480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be212b601080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510cebfe23e321406fd60078eb4c2356137dd50036597db267cf61642409276f20ad4b152f91c344bd63ac691bad66e04e228a8b58dca293ff0bd10f8aef6dfbcecae49e32b09d89e10b771a6c01628628596a95e126b04763560c66c0f12b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510a4caa532321418e69cc672973992bb5f76d049a5b2c5ddf77436380142409ed2b74fa835296d552e68c439dd4ee3fa94fb197282edcc1cc815c863ca42a2c9a73475ff6be9064371a61655a3c31d2f0acc89c3a4489ad4c2671aef52360512b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510a69eca2f3214344c39bb8f4512d6cab1f6aafac1811ef9d8afdf38024240de2768ead90011bcbb1914abc1572749ab7b81382eb81cff3b41c56edc12470a7b8a4d61f8b4ca7b2cb7e24706edd219455796b4db74cd36965859f91dc8910312b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510dcdd833b321437ef19af29679b368d2b9e9de3f8769b357866763803424072ddfe0aeb13616b3f17eb60b19a923ec51fcc726625094aa069255c829c8cdd9e242080a1e559b0030fe9a0db19fd34e392bd78df12a9caff9f2b811bc1ac0a12b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510e9f2f859321462633d9db7ed78e951f79913fdc8231aa77ec12b38044240f5f61c640ab2402b44936de0d24e7b439df78bc3ef15467ecb29b92ece4aa0550790d5ce80761f2ac4b0e3283969725c42343749d9b44b179b2d4fced66c5d0412b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510ff90f55532147b343e041ca130000a8bc00c35152bd7e774003738054240df6e298b3efd42eb536e68a0210bc921e8b5dc145fe965f63f4d3490064f239f2a54a6db16c96086e4ae52280c04ad8b32b44f5ff3d41f0c364949ccb628c50312b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510cad7c931321491844d296bd8e591448efc65fd6ad51a888d58fa3806424030298627da1afd28229aac150f553724b594989e59136d6a175d84e45a4dee344ff9e0eeb69fdf29abb6d833adc3e1ccdc87b2a65019ef5fb627c44d9d132c0012b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510c8c296323214b3727172ce6473bc780298a2d66c12f1a14f5b2a38074240918491100730b4523f0c85409f6d1cca9ebc4b8ca6df8d55fe3d85158fa43286608693c50332953e1d3b93e3e78b24e158d6a2275ce8c6c7c07a7a646a19200312b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef2051086f1a2403214b6f20c7faa2b2f6f24518fa02b71cb5f4a09fba338084240ca59c9fc7f6ab660e9970fc03e5ed588ccb8be43fe5a3e8450287b726f29d039e53fe888438f178ac63c3d2ca969cd8c2fbc8606f067634339b6a94a7382960212b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef2051080efbb543214e0dd72609cc106210d1aa13936cb67b93a0aee2138094240e787a21f5cb7052624160759a9d379dd9db144f2b498bca026375c9ce8ecdc2a0936af1c309b3a0f686c92bf5578b595a4ca99036a19c9fc50d3718fd454b30012b801080210e38bf01f22480a207eaabf7df1081377e06e08efe7ad17974049380bdd65a9b053c099ef80ff6e6f122408011220d153cc308d9cb96ca43ffeceaae1ee85794c83d17408ff76cfee92f5e91d0be22a0b08e291aef20510ddf8d85a3214fc3108dc3814888f4187452182bc1baf83b71bc9380a4240d51ea31f6449eed71de22339722af1edbb0b21401037d85882b32a2ed8ae9127f2df4d1da2092729e582812856227ed6cdf98a3f60203d1ff80bd635fb03bb0912a4070a4f0a1406fd60078eb4c2356137dd50036597db267cf61612251624de6420e17cbe9c20cdcfdf876b3b12978d3264a007fcaaa71c4cdb701d9ebc0323f44f1880d0dbc3f4022080e0ebdaf2e2ffffff010a4b0a1418e69cc672973992bb5f76d049a5b2c5ddf7743612251624de6420184e7b103d34c41003f9b864d5f8c1adda9bd0436b253bb3c844bc739c1e77c91880d0dbc3f4022080d0dbc3f4020a4b0a14344c39bb8f4512d6cab1f6aafac1811ef9d8afdf12251624de64204d420aea843e92a0cfe69d89696dff6827769f9cb52a249af537ce89bf2a4b741880d0dbc3f4022080d0dbc3f4020a4b0a1437ef19af29679b368d2b9e9de3f8769b3578667612251624de6420bd03de9f8ab29e2800094e153fac6f696cfa512536c9c2f804dcb2c2c4e4aed61880d0dbc3f4022080d0dbc3f4020a4b0a1462633d9db7ed78e951f79913fdc8231aa77ec12b12251624de64208f4a74a07351895ddf373057b98fae6dfaf2cd21f37a063e19601078fe470d531880d0dbc3f4022080d0dbc3f4020a4b0a147b343e041ca130000a8bc00c35152bd7e774003712251624de64204a5d4753eb79f92e80efe22df7aca4f666a4f44bf81c536c4a09d4b9c5b654b51880d0dbc3f4022080d0dbc3f4020a4b0a1491844d296bd8e591448efc65fd6ad51a888d58fa12251624de6420c80e9abef7ff439c10c68fe8f1303deddfc527718c3b37d8ba6807446e3c827a1880d0dbc3f4022080d0dbc3f4020a4b0a14b3727172ce6473bc780298a2d66c12f1a14f5b2a12251624de64209142afcc691b7cc05d26c7b0be0c8b46418294171730e079f384fde2fa50bafc1880d0dbc3f4022080d0dbc3f4020a4b0a14b6f20c7faa2b2f6f24518fa02b71cb5f4a09fba312251624de642049b288e4ebbb3a281c2d546fc30253d5baf08993b6e5d295fb787a5b314a298e1880d0dbc3f4022080d0dbc3f4020a4b0a14e0dd72609cc106210d1aa13936cb67b93a0aee2112251624de642004224339688f012e649de48e241880092eaa8f6aa0f4f14bfcf9e0c76917c0b61880d0dbc3f4022080d0dbc3f4020a4b0a14fc3108dc3814888f4187452182bc1baf83b71bc912251624de64204034b37ceda8a0bf13b1abaeee7a8f9383542099a554d219b93d0ce69e3970e81880d0dbc3f4022080d0dbc3f402124f0a1406fd60078eb4c2356137dd50036597db267cf61612251624de6420e17cbe9c20cdcfdf876b3b12978d3264a007fcaaa71c4cdb701d9ebc0323f44f1880d0dbc3f4022080e0ebdaf2e2ffffff011aa4070a4f0a1406fd60078eb4c2356137dd50036597db267cf61612251624de6420e17cbe9c20cdcfdf876b3b12978d3264a007fcaaa71c4cdb701d9ebc0323f44f1880d0dbc3f4022080e0ebdaf2e2ffffff010a4b0a1418e69cc672973992bb5f76d049a5b2c5ddf7743612251624de6420184e7b103d34c41003f9b864d5f8c1adda9bd0436b253bb3c844bc739c1e77c91880d0dbc3f4022080d0dbc3f4020a4b0a14344c39bb8f4512d6cab1f6aafac1811ef9d8afdf12251624de64204d420aea843e92a0cfe69d89696dff6827769f9cb52a249af537ce89bf2a4b741880d0dbc3f4022080d0dbc3f4020a4b0a1437ef19af29679b368d2b9e9de3f8769b3578667612251624de6420bd03de9f8ab29e2800094e153fac6f696cfa512536c9c2f804dcb2c2c4e4aed61880d0dbc3f4022080d0dbc3f4020a4b0a1462633d9db7ed78e951f79913fdc8231aa77ec12b12251624de64208f4a74a07351895ddf373057b98fae6dfaf2cd21f37a063e19601078fe470d531880d0dbc3f4022080d0dbc3f4020a4b0a147b343e041ca130000a8bc00c35152bd7e774003712251624de64204a5d4753eb79f92e80efe22df7aca4f666a4f44bf81c536c4a09d4b9c5b654b51880d0dbc3f4022080d0dbc3f4020a4b0a1491844d296bd8e591448efc65fd6ad51a888d58fa12251624de6420c80e9abef7ff439c10c68fe8f1303deddfc527718c3b37d8ba6807446e3c827a1880d0dbc3f4022080d0dbc3f4020a4b0a14b3727172ce6473bc780298a2d66c12f1a14f5b2a12251624de64209142afcc691b7cc05d26c7b0be0c8b46418294171730e079f384fde2fa50bafc1880d0dbc3f4022080d0dbc3f4020a4b0a14b6f20c7faa2b2f6f24518fa02b71cb5f4a09fba312251624de642049b288e4ebbb3a281c2d546fc30253d5baf08993b6e5d295fb787a5b314a298e1880d0dbc3f4022080d0dbc3f4020a4b0a14e0dd72609cc106210d1aa13936cb67b93a0aee2112251624de642004224339688f012e649de48e241880092eaa8f6aa0f4f14bfcf9e0c76917c0b61880d0dbc3f4022080d0dbc3f4020a4b0a14fc3108dc3814888f4187452182bc1baf83b71bc912251624de64204034b37ceda8a0bf13b1abaeee7a8f9383542099a554d219b93d0ce69e3970e81880d0dbc3f4022080d0dbc3f402124f0a1406fd60078eb4c2356137dd50036597db267cf61612251624de6420e17cbe9c20cdcfdf876b3b12978d3264a007fcaaa71c4cdb701d9ebc0323f44f1880d0dbc3f4022080e0ebdaf2e2ffffff01").unwrap();
        let res = tendermint_header_validation_run(&Bytes::from(input), 3_000u64).unwrap();

        let gas = res.0;
        assert_eq!(gas, 3_000u64);

        let output = hex::encode(res.1);
        assert_eq!(output, "000000000000000000000000000000000000000000000000000000000000022042696e616e63652d436861696e2d4e696c6500000000000000000000000000000000000003fc05e3a3e248bc209955054d880e4d89ff3c0419c0cd77681f4b4c6649ead5545054b980d9ab0fc10d18ca0e0832d5f4c063c5489ec1443dfb738252d038a82131b27ae17cbe9c20cdcfdf876b3b12978d3264a007fcaaa71c4cdb701d9ebc0323f44f000000174876e800184e7b103d34c41003f9b864d5f8c1adda9bd0436b253bb3c844bc739c1e77c9000000174876e8004d420aea843e92a0cfe69d89696dff6827769f9cb52a249af537ce89bf2a4b74000000174876e800bd03de9f8ab29e2800094e153fac6f696cfa512536c9c2f804dcb2c2c4e4aed6000000174876e8008f4a74a07351895ddf373057b98fae6dfaf2cd21f37a063e19601078fe470d53000000174876e8004a5d4753eb79f92e80efe22df7aca4f666a4f44bf81c536c4a09d4b9c5b654b5000000174876e800c80e9abef7ff439c10c68fe8f1303deddfc527718c3b37d8ba6807446e3c827a000000174876e8009142afcc691b7cc05d26c7b0be0c8b46418294171730e079f384fde2fa50bafc000000174876e80049b288e4ebbb3a281c2d546fc30253d5baf08993b6e5d295fb787a5b314a298e000000174876e80004224339688f012e649de48e241880092eaa8f6aa0f4f14bfcf9e0c76917c0b6000000174876e8004034b37ceda8a0bf13b1abaeee7a8f9383542099a554d219b93d0ce69e3970e8000000174876e800");
    }
}
