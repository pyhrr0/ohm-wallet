// @generated automatically by Diesel CLI.

diesel::table! {
    cosigner (id) {
        id -> Integer,
        uuid -> Text,
        #[sql_name = "type"]
        type_ -> SmallInt,
        email_address -> Nullable<Text>,
        xpub -> Text,
        xprv -> Nullable<Text>,
        creation_time -> Timestamp,
        wallet_uuid -> Nullable<Text>,
    }
}

diesel::table! {
    psbt (id) {
        id -> Integer,
        uuid -> Text,
        base64 -> Text,
        creation_time -> Timestamp,
        wallet_uuid -> Text,
    }
}

diesel::table! {
    wallet (id) {
        id -> Integer,
        uuid -> Text,
        address_type -> SmallInt,
        network -> SmallInt,
        receive_descriptor -> Text,
        receive_descriptor_watch_only -> Text,
        receive_address_index -> BigInt,
        receive_address -> Text,
        change_descriptor -> Text,
        change_descriptor_watch_only -> Text,
        change_address_index -> BigInt,
        change_address -> Text,
        required_signatures -> SmallInt,
        balance -> Text,
        creation_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cosigner,
    psbt,
    wallet,
);
