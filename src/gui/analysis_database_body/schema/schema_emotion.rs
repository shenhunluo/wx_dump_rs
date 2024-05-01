diesel::table! {
    Name2ID (UsrName) {
        UsrName -> Nullable<Text>,
    }
}

diesel::table! {
    EmotionItem (localId) {
        localId -> Nullable<Integer>,
        ProductId -> Nullable<Integer>,
        MD5 -> Text,
        Reserved0 -> Nullable<Integer>,
        Reserved1 -> Nullable<Text>,
        Type -> Nullable<Integer>,
        AppId -> Nullable<Text>,
        FromUrl -> Nullable<Text>,
        Thumb -> Nullable<Binary>,
        Data -> Nullable<Binary>,
        Reserved2 -> Nullable<Integer>,
        Reserved3 -> Nullable<Text>,
        Reserved4 -> Nullable<Integer>,
    }
}
