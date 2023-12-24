diesel::table! {
    Media (Key) {
        Key -> Nullable<Text>,
        Reserved0 -> Nullable<BigInt>,
        Buf -> Nullable<Binary>,
        Reserved1 -> Nullable<Integer>,
        Reserved2 -> Nullable<Text>,
    }
}