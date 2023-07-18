// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use std::borrow::Cow;
use tw_coin_entry::coin_entry_ext::CoinEntryExt;
use tw_coin_entry::error::SigningErrorType;
use tw_coin_entry::test_helpers::dummy_context::DummyCoinContext;
use tw_encoding::hex;
use tw_encoding::hex::ToHex;
use tw_ethereum::entry::EthereumEntry;
use tw_ethereum::modules::signer::Signer;
use tw_number::U256;
use tw_proto::serialize;
use tw_proto::Ethereum::Proto;
use tw_proto::Ethereum::Proto::TransactionMode;

#[test]
fn test_sign_transaction_non_typed_erc20_transfer() {
    let private =
        hex::decode("0x4646464646464646464646464646464646464646464646464646464646464646").unwrap();

    let erc20_transfer = Proto::mod_Transaction::ERC20Transfer {
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        amount: U256::encode_be_compact(2_000_000_000_000_000_000),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(0x34),
        tx_mode: TransactionMode::Legacy,
        // 42000000000
        gas_price: U256::encode_be_compact(0x09_c765_2400),
        // 78009
        gas_limit: U256::encode_be_compact(0x01_30B9),
        // DAI
        to_address: "0x6b175474e89094c44da98b954eedeac495271d0f".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc20_transfer(
                erc20_transfer,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "f8ab808509c7652400830130b9946b175474e89094c44da98b954eedeac495271d0f80b844a9059cbb0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000001bc16d674ec80000818ba0c34040ff76f6d5e397b54b47f7fa2b3a7213f3c2a39a750260211fa15249ae8aa01ac5061e9bcf05aebef461864662652f25c45ee99240e3bb91b31f456208a6cd";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "b3525019dc367d3ecac48905f9a95ff3550c25a24823db765f92cae2dec7ebfd"
    );
}

#[test]
fn test_sign_transaction_non_typed_native() {
    let private =
        hex::decode("0x4646464646464646464646464646464646464646464646464646464646464646").unwrap();

    let transfer = Proto::mod_Transaction::Transfer {
        amount: U256::encode_be_compact(1_000_000_000_000_000_000),
        data: Cow::default(),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(9),
        gas_price: U256::encode_be_compact(20_000_000_000),
        gas_limit: U256::encode_be_compact(21_000),
        to_address: "0x3535353535353535353535353535353535353535".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::transfer(transfer),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "f86c098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a76400008025a028ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276a067cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.r, false),
        "28ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276"
    );
    assert_eq!(
        hex::encode(output.s, false),
        "67cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83"
    );
    assert_eq!(hex::encode(output.v, false), "25");

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "daf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53"
    );
}

#[test]
fn test_sign_transaction_non_typed_erc20_approve() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let approve = Proto::mod_Transaction::ERC20Approve {
        // DAI
        spender: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        amount: U256::encode_be_compact(2_000_000_000_000_000_000),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(0),
        // 0x09c7652400
        gas_price: U256::encode_be_compact(42_000_000_000),
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        to_address: "0x6b175474e89094c44da98b954eedeac495271d0f".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc20_approve(
                approve,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "f8aa808509c7652400830130b9946b175474e89094c44da98b954eedeac495271d0f80b844095ea7b30000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000001bc16d674ec8000025a0d8136d66da1e0ba8c7208d5c4f143167f54b89a0fe2e23440653bcca28b34dc1a049222a79339f1a9e4641cb4ad805c49c225ae704299ffc10627bf41c035c464a";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.r, false),
        "d8136d66da1e0ba8c7208d5c4f143167f54b89a0fe2e23440653bcca28b34dc1"
    );
    assert_eq!(
        hex::encode(output.s, false),
        "49222a79339f1a9e4641cb4ad805c49c225ae704299ffc10627bf41c035c464a"
    );
    assert_eq!(hex::encode(output.v, false), "25");

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "fe34a2b97f583db2d4baca3753f105d70dcf2d9800ddd0247900a026b9de6183"
    );
}

#[test]
fn test_sign_transaction_non_typed_contract_generic() {
    let private =
        hex::decode("0x4646464646464646464646464646464646464646464646464646464646464646").unwrap();
    let call_data = "60a060405260046060527f48302e31000000000000000000000000000000000000000000000000000000006080526006805460008290527f48302e310000000000000000000000000000000000000000000000000000000882556100b5907ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f602060026001841615610100026000190190931692909204601f01919091048101905b8082111561017957600081556001016100a1565b505060405161094b38038061094b83398101604052808051906020019091908051820191906020018051906020019091908051820191906020015050836000600050600033600160a060020a0316815260200190815260200160002060005081905550836002600050819055508260036000509080519060200190828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f1061017d57805160ff19168380011785555b506101ad9291506100a1565b5090565b8280016001018555821561016d579182015b8281111561016d57825182600050559160200191906001019061018f565b50506004805460ff19168317905560058054825160008390527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0602060026001851615610100026000190190941693909304601f90810184900482019386019083901061022d57805160ff19168380011785555b5061025d9291506100a1565b82800160010185558215610221579182015b8281111561022157825182600050559160200191906001019061023f565b5050505050506106da806102716000396000f36060604052361561008d5760e060020a600035046306fdde038114610095578063095ea7b3146100f357806318160ddd1461016857806323b872dd14610171578063313ce5671461025c57806354fd4d501461026857806370a08231146102c657806395d89b41146102f4578063a9059cbb14610352578063cae9ca51146103f7578063dd62ed3e146105be575b6105f2610002565b6040805160038054602060026001831615610100026000190190921691909104601f81018290048202840182019094528383526105f493908301828280156106b75780601f1061068c576101008083540402835291602001916106b7565b61066260043560243533600160a060020a03908116600081815260016020908152604080832094871680845294825280832086905580518681529051929493927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929181900390910190a35060015b92915050565b6102e260025481565b610662600435602435604435600160a060020a0383166000908152602081905260408120548290108015906101c4575060016020908152604080832033600160a060020a03168452909152812054829010155b80156101d05750600082115b156106bf57600160a060020a0383811660008181526020818152604080832080548801905588851680845281842080548990039055600183528184203390961684529482529182902080548790039055815186815291519293927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9281900390910190a35060016106c3565b61067660045460ff1681565b6040805160068054602060026001831615610100026000190190921691909104601f81018290048202840182019094528383526105f493908301828280156106b75780601f1061068c576101008083540402835291602001916106b7565b600160a060020a03600435166000908152602081905260409020545b60408051918252519081900360200190f35b6105f46005805460408051602060026001851615610100026000190190941693909304601f810184900484028201840190925281815292918301828280156106b75780601f1061068c576101008083540402835291602001916106b7565b61066260043560243533600160a060020a03166000908152602081905260408120548290108015906103845750600082115b156106ca5733600160a060020a0390811660008181526020818152604080832080548890039055938716808352918490208054870190558351868152935191937fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef929081900390910190a3506001610162565b604080516020604435600481810135601f810184900484028501840190955284845261066294813594602480359593946064949293910191819084018382808284375094965050505050505033600160a060020a03908116600081815260016020908152604080832094881680845294825280832087905580518781529051929493927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929181900390910190a383600160a060020a031660405180807f72656365697665417070726f76616c28616464726573732c75696e743235362c81526020017f616464726573732c627974657329000000000000000000000000000000000000815260200150602e019050604051809103902060e060020a9004338530866040518560e060020a0281526004018085600160a060020a0316815260200184815260200183600160a060020a031681526020018280519060200190808383829060006004602084601f0104600f02600301f150905090810190601f1680156105965780820380516001836020036101000a031916815260200191505b509450505050506000604051808303816000876161da5a03f19250505015156106d257610002565b6102e2600435602435600160a060020a03828116600090815260016020908152604080832093851683529290522054610162565b005b60405180806020018281038252838181518152602001915080519060200190808383829060006004602084601f0104600f02600301f150905090810190601f1680156106545780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b604080519115158252519081900360200190f35b6040805160ff9092168252519081900360200190f35b820191906000526020600020905b81548152906001019060200180831161069a57829003601f168201915b505050505081565b5060005b9392505050565b506000610162565b5060016106c35600000000000000000000000000000000000000000000000000000000000003e80000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000754204275636b73000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003544f540000000000000000000000000000000000000000000000000000000000";

    let contract = Proto::mod_Transaction::ContractGeneric {
        amount: U256::encode_be_compact(0),
        data: hex::decode(call_data).unwrap().into(),
    };

    // `tx_mode` is not set, Legacy is the default.
    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(11),
        gas_price: U256::encode_be_compact(20_000_000_000),
        gas_limit: U256::encode_be_compact(1_000_000),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::contract_generic(
                contract,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "5d2556f7d0e629dc6ce9dbc8a205853a7b89c136791840a39765e34cb5e3466a"
    );

    let expected = "f90a9e0b8504a817c800830f42408080b90a4b60a060405260046060527f48302e31000000000000000000000000000000000000000000000000000000006080526006805460008290527f48302e310000000000000000000000000000000000000000000000000000000882556100b5907ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f602060026001841615610100026000190190931692909204601f01919091048101905b8082111561017957600081556001016100a1565b505060405161094b38038061094b83398101604052808051906020019091908051820191906020018051906020019091908051820191906020015050836000600050600033600160a060020a0316815260200190815260200160002060005081905550836002600050819055508260036000509080519060200190828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f1061017d57805160ff19168380011785555b506101ad9291506100a1565b5090565b8280016001018555821561016d579182015b8281111561016d57825182600050559160200191906001019061018f565b50506004805460ff19168317905560058054825160008390527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0602060026001851615610100026000190190941693909304601f90810184900482019386019083901061022d57805160ff19168380011785555b5061025d9291506100a1565b82800160010185558215610221579182015b8281111561022157825182600050559160200191906001019061023f565b5050505050506106da806102716000396000f36060604052361561008d5760e060020a600035046306fdde038114610095578063095ea7b3146100f357806318160ddd1461016857806323b872dd14610171578063313ce5671461025c57806354fd4d501461026857806370a08231146102c657806395d89b41146102f4578063a9059cbb14610352578063cae9ca51146103f7578063dd62ed3e146105be575b6105f2610002565b6040805160038054602060026001831615610100026000190190921691909104601f81018290048202840182019094528383526105f493908301828280156106b75780601f1061068c576101008083540402835291602001916106b7565b61066260043560243533600160a060020a03908116600081815260016020908152604080832094871680845294825280832086905580518681529051929493927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929181900390910190a35060015b92915050565b6102e260025481565b610662600435602435604435600160a060020a0383166000908152602081905260408120548290108015906101c4575060016020908152604080832033600160a060020a03168452909152812054829010155b80156101d05750600082115b156106bf57600160a060020a0383811660008181526020818152604080832080548801905588851680845281842080548990039055600183528184203390961684529482529182902080548790039055815186815291519293927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9281900390910190a35060016106c3565b61067660045460ff1681565b6040805160068054602060026001831615610100026000190190921691909104601f81018290048202840182019094528383526105f493908301828280156106b75780601f1061068c576101008083540402835291602001916106b7565b600160a060020a03600435166000908152602081905260409020545b60408051918252519081900360200190f35b6105f46005805460408051602060026001851615610100026000190190941693909304601f810184900484028201840190925281815292918301828280156106b75780601f1061068c576101008083540402835291602001916106b7565b61066260043560243533600160a060020a03166000908152602081905260408120548290108015906103845750600082115b156106ca5733600160a060020a0390811660008181526020818152604080832080548890039055938716808352918490208054870190558351868152935191937fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef929081900390910190a3506001610162565b604080516020604435600481810135601f810184900484028501840190955284845261066294813594602480359593946064949293910191819084018382808284375094965050505050505033600160a060020a03908116600081815260016020908152604080832094881680845294825280832087905580518781529051929493927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929181900390910190a383600160a060020a031660405180807f72656365697665417070726f76616c28616464726573732c75696e743235362c81526020017f616464726573732c627974657329000000000000000000000000000000000000815260200150602e019050604051809103902060e060020a9004338530866040518560e060020a0281526004018085600160a060020a0316815260200184815260200183600160a060020a031681526020018280519060200190808383829060006004602084601f0104600f02600301f150905090810190601f1680156105965780820380516001836020036101000a031916815260200191505b509450505050506000604051808303816000876161da5a03f19250505015156106d257610002565b6102e2600435602435600160a060020a03828116600090815260016020908152604080832093851683529290522054610162565b005b60405180806020018281038252838181518152602001915080519060200190808383829060006004602084601f0104600f02600301f150905090810190601f1680156106545780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b604080519115158252519081900360200190f35b6040805160ff9092168252519081900360200190f35b820191906000526020600020905b81548152906001019060200180831161069a57829003601f168201915b505050505081565b5060005b9392505050565b506000610162565b5060016106c35600000000000000000000000000000000000000000000000000000000000003e80000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000754204275636b73000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003544f54000000000000000000000000000000000000000000000000000000000026a042556c4f2a3f4e4e639cca524d1da70e60881417d4643e5382ed110a52719eafa0172f591a2a763d0bd6b13d042d8c5eb66e87f129c9dc77ada66b6041012db2b3";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(hex::encode(output.data, false), call_data);
}

// https://ropsten.etherscan.io/tx/0x14429509307efebfdaa05227d84c147450d168c68539351fbc01ed87c916ab2e
#[test]
fn test_sign_transaction_eip1559_native_transfer() {
    let private =
        hex::decode("4f96ed80e9a7555a6f74b3d658afdd9c756b0a40d4ca30c42c2039eb449bb904").unwrap();

    let transfer = Proto::mod_Transaction::Transfer {
        amount: U256::encode_be_compact(543_210_987_654_321),
        data: Cow::default(),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(3),
        nonce: U256::encode_be_compact(6),
        tx_mode: TransactionMode::Enveloped,
        gas_limit: U256::encode_be_compact(21_100),
        max_inclusion_fee_per_gas: U256::encode_be_compact(2_000_000_000),
        max_fee_per_gas: U256::encode_be_compact(3_000_000_000),
        to_address: "0xB9F5771C27664bF2282D98E09D7F50cEc7cB01a7".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::transfer(transfer),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "02f8710306847735940084b2d05e0082526c94b9f5771c27664bf2282d98e09d7f50cec7cb01a78701ee0c29f50cb180c080a092c336138f7d0231fe9422bb30ee9ef10bf222761fe9e04442e3a11e88880c64a06487026011dae03dc281bc21c7d7ede5c2226d197befb813a4ecad686b559e58";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        output.r.to_hex(),
        "92c336138f7d0231fe9422bb30ee9ef10bf222761fe9e04442e3a11e88880c64"
    );
    assert_eq!(
        output.s.to_hex(),
        "6487026011dae03dc281bc21c7d7ede5c2226d197befb813a4ecad686b559e58"
    );
    assert_eq!(output.v.to_hex(), "00");

    assert_eq!(
        output.pre_hash.to_hex(),
        "6468eb103d51c9a683b51818fdb73390151c9973831d2cfb4e9587ad54273155"
    );
}

#[test]
fn test_sign_transaction_eip1559_erc20_transfer() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc20_transfer = Proto::mod_Transaction::ERC20Transfer {
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        amount: U256::encode_be_compact(2_000_000_000_000_000_000),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(0),
        tx_mode: TransactionMode::Enveloped,
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        // 0x77359400
        max_inclusion_fee_per_gas: U256::encode_be_compact(2_000_000_000),
        // 0xB2D05E00
        max_fee_per_gas: U256::encode_be_compact(3_000_000_000),
        // DAI
        to_address: "0x6b175474e89094c44da98b954eedeac495271d0f".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc20_transfer(
                erc20_transfer,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "02f8b00180847735940084b2d05e00830130b9946b175474e89094c44da98b954eedeac495271d0f80b844a9059cbb0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000001bc16d674ec80000c080a0adfcfdf98d4ed35a8967a0c1d78b42adb7c5d831cf5a3272654ec8f8bcd7be2ea011641e065684f6aa476f4fd250aa46cd0b44eccdb0a6e1650d658d1998684cdf";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "aa0ec30afa12acb48a080aa7157254193eeb2a4d248538b0747535baab98141f"
    );
}

#[test]
fn test_sign_transaction_eip1559_erc20_approve() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc20_approve = Proto::mod_Transaction::ERC20Approve {
        spender: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        amount: U256::encode_be_compact(2_000_000_000_000_000_000),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(0),
        tx_mode: TransactionMode::Enveloped,
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        // 0x77359400
        max_inclusion_fee_per_gas: U256::encode_be_compact(2_000_000_000),
        // 0xB2D05E00
        max_fee_per_gas: U256::encode_be_compact(3_000_000_000),
        // DAI
        to_address: "0x6b175474e89094c44da98b954eedeac495271d0f".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc20_approve(
                erc20_approve,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "02f8b00180847735940084b2d05e00830130b9946b175474e89094c44da98b954eedeac495271d0f80b844095ea7b30000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000001bc16d674ec80000c080a05a43dda3dc193480ee532a5ed67ba8fbd2e3afb9eee218f4fb955b415d592925a01300e5b5f51c8cd5bf80f018cea3fb347fae589e65355068ac44ffc996313c60";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "bed87d402cc536fac3dbb346eac221f5ee783e04f0a7e3713817e55a78d08065"
    );
}

#[test]
fn test_sign_transaction_eip1559_erc721_transfer() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc20_approve = Proto::mod_Transaction::ERC721Transfer {
        from: "0x718046867b5b1782379a14eA4fc0c9b724DA94Fc".into(),
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        token_id: hex::decode("23c47ee5").unwrap().into(),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(0),
        tx_mode: TransactionMode::Enveloped,
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        // 0x77359400
        max_inclusion_fee_per_gas: U256::encode_be_compact(2_000_000_000),
        // 0xB2D05E00
        max_fee_per_gas: U256::encode_be_compact(3_000_000_000),
        to_address: "0x4e45e92ed38f885d39a733c14f1817217a89d425".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc721_transfer(
                erc20_approve,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "02f8d00180847735940084b2d05e00830130b9944e45e92ed38f885d39a733c14f1817217a89d42580b86423b872dd000000000000000000000000718046867b5b1782379a14ea4fc0c9b724da94fc0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000000000000023c47ee5c080a0dbd591d1eac39bad62d7c158d5e1d55e7014d2218998f8980462e2f283f42d4aa05acadb904484a0fb5526a4c64b8addb8aac4f6548f90199e40eb787b79faed4a";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "6089b11574c558e717fd25fe84bb7525bc32408f7124c2f199e26dfb0845abdd"
    );
}

#[test]
fn test_sign_transaction_eip1559_erc1155_transfer() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc1155_approve = Proto::mod_Transaction::ERC1155Transfer {
        from: "0x718046867b5b1782379a14eA4fc0c9b724DA94Fc".into(),
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        token_id: hex::decode("23c47ee5").unwrap().into(),
        value: U256::encode_be_compact(2_000_000_000_000_000_000),
        data: hex::decode("01020304").unwrap().into(),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(0),
        tx_mode: TransactionMode::Enveloped,
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        // 0x77359400
        max_inclusion_fee_per_gas: U256::encode_be_compact(2_000_000_000),
        // 0xB2D05E00
        max_fee_per_gas: U256::encode_be_compact(3_000_000_000),
        to_address: "0x4e45e92ed38f885d39a733c14f1817217a89d425".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc1155_transfer(
                erc1155_approve,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "02f901500180847735940084b2d05e00830130b9944e45e92ed38f885d39a733c14f1817217a89d42580b8e4f242432a000000000000000000000000718046867b5b1782379a14ea4fc0c9b724da94fc0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000000000000023c47ee50000000000000000000000000000000000000000000000001bc16d674ec8000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000040102030400000000000000000000000000000000000000000000000000000000c080a0533df41dda5540c57257b7fe89c29cefff0155c333e063220df2bf9680fcc15aa036a844fd20de5a51de96ceaaf078558e87d86426a4a5d4b215ee1fd0fa397f8a";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "d18f2ac6b3bff71457bfed1fc897b42a5a4220b1589eadbbace8d3def656f914"
    );
}

#[test]
fn test_sign_transaction_non_typed_erc20_transfer_as_contract_generic() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    // payload: transfer(0x5322b34c88ed0691971bf52a7047448f0f4efc84, 2000000000000000000)
    let contract_data = hex::decode("0xa9059cbb0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000001bc16d674ec80000").unwrap();
    let contract = Proto::mod_Transaction::ContractGeneric {
        amount: U256::encode_be_compact(0),
        data: contract_data.into(),
    };

    // `tx_mode` is not set, Legacy is the default.
    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        nonce: U256::encode_be_compact(0),
        // 0x09c7652400
        gas_price: U256::encode_be_compact(42_000_000_000),
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        to_address: "0x6b175474e89094c44da98b954eedeac495271d0f".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::contract_generic(
                contract,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "f8aa808509c7652400830130b9946b175474e89094c44da98b954eedeac495271d0f80b844a9059cbb0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000001bc16d674ec8000025a0724c62ad4fbf47346b02de06e603e013f26f26b56fdc0be7ba3d6273401d98cea0032131cae15da7ddcda66963e8bef51ca0d9962bfef0547d3f02597a4a58c931";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "3a3fc6df8815e15874cc8d6c65f88ea0643b375e5b22726269d187035a5cb486"
    );
}

#[test]
fn test_sign_transaction_non_typed_erc20_transfer_invalid_address() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc20_transfer = Proto::mod_Transaction::ERC20Transfer {
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        amount: U256::encode_be_compact(2_000_000_000_000_000_000),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        tx_mode: TransactionMode::Legacy,
        // 42000000000
        gas_price: U256::encode_be_compact(0x09_c765_2400),
        // 78009
        gas_limit: U256::encode_be_compact(0x01_30B9),
        // DAI
        to_address: "0xdeadbeef".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc20_transfer(
                erc20_transfer,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::Error_invalid_address);
    assert!(!output.error_message.is_empty());
}

#[test]
fn test_sign_transaction_non_typed_erc721_transfer() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc721_transfer = Proto::mod_Transaction::ERC721Transfer {
        from: "0x718046867b5b1782379a14eA4fc0c9b724DA94Fc".into(),
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        token_id: hex::decode("23c47ee5").unwrap().into(),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        tx_mode: TransactionMode::Legacy,
        // 0x09c7652400
        gas_price: U256::encode_be_compact(42_000_000_000),
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        to_address: "0x4e45e92ed38f885d39a733c14f1817217a89d425".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc721_transfer(
                erc721_transfer,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "f8ca808509c7652400830130b9944e45e92ed38f885d39a733c14f1817217a89d42580b86423b872dd000000000000000000000000718046867b5b1782379a14ea4fc0c9b724da94fc0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000000000000023c47ee526a0d38440a4dc140a4100d301eb49fcc35b64439e27d1d8dd9b55823dca04e6e659a03b5f56a57feabc3406f123d6f8198cd7d7e2ced7e2d58d375f076952ecd9ce88";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "5de6e2bc3a0e95a32c2a6075bec622feccb8e8dab8fa564de7468416b44eff32"
    );

    let expected_data = "23b872dd000000000000000000000000718046867b5b1782379a14ea4fc0c9b724da94fc0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000000000000023c47ee5";
    assert_eq!(hex::encode(output.data, false), expected_data);
}

#[test]
fn test_sign_transaction_non_typed_erc1155_transfer() {
    let private =
        hex::decode("0x608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151").unwrap();

    let erc1155_transfer = Proto::mod_Transaction::ERC1155Transfer {
        from: "0x718046867b5b1782379a14eA4fc0c9b724DA94Fc".into(),
        to: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".into(),
        token_id: hex::decode("23c47ee5").unwrap().into(),
        value: U256::encode_be_compact(2_000_000_000_000_000_000),
        data: hex::decode("01020304").unwrap().into(),
    };

    let input = Proto::SigningInput {
        chain_id: U256::encode_be_compact(1),
        tx_mode: TransactionMode::Legacy,
        // 0x09c7652400
        gas_price: U256::encode_be_compact(42_000_000_000),
        // 0x130B9
        gas_limit: U256::encode_be_compact(78_009),
        to_address: "0x4e45e92ed38f885d39a733c14f1817217a89d425".into(),
        transaction: Some(Proto::Transaction {
            transaction_oneof: Proto::mod_Transaction::OneOftransaction_oneof::erc1155_transfer(
                erc1155_transfer,
            ),
        }),
        private_key: private.into(),
        ..Proto::SigningInput::default()
    };

    let output = Signer::sign_proto(input);
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected = "f9014a808509c7652400830130b9944e45e92ed38f885d39a733c14f1817217a89d42580b8e4f242432a000000000000000000000000718046867b5b1782379a14ea4fc0c9b724da94fc0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000000000000023c47ee50000000000000000000000000000000000000000000000001bc16d674ec8000000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000004010203040000000000000000000000000000000000000000000000000000000026a010315488201ac801ce346bffd1570de147615462d7e7db3cf08cf558465c6b79a06643943b24593bc3904a9fda63bb169881730994c973ab80f07d66a698064573";
    assert_eq!(hex::encode(output.encoded, false), expected);

    assert_eq!(
        hex::encode(output.pre_hash, false),
        "d183270df1081772f867c9e6d16601028dd7faa6103e5d7286f1f5e4bde4f547"
    );

    let expected_data = "f242432a000000000000000000000000718046867b5b1782379a14ea4fc0c9b724da94fc0000000000000000000000005322b34c88ed0691971bf52a7047448f0f4efc840000000000000000000000000000000000000000000000000000000023c47ee50000000000000000000000000000000000000000000000001bc16d674ec8000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000040102030400000000000000000000000000000000000000000000000000000000";
    assert_eq!(hex::encode(output.data, false), expected_data);
}

#[test]
fn test_plan_not_supported() {
    let input = Proto::SigningInput::default();
    let input_data = serialize(&input).unwrap();
    let maybe_plan = EthereumEntry
        .plan(&DummyCoinContext, &input_data)
        .expect("!plan");
    assert_eq!(maybe_plan, None, "Ethereum does not support plan()");
}
