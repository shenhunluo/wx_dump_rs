diesel::table! {
    DBInfo (tableIndex) {
        tableIndex -> Nullable<Integer>,
        tableVersion -> Nullable<Integer>,
        tableDesc -> Nullable<Text>,
    }
}

diesel::table! {
    MSG (localId) {
        localId -> Nullable<Integer>,
        TalkerId -> Nullable<Integer>,
        MsgSvrID -> Nullable<Integer>,
        Type -> Nullable<Integer>,
        SubType -> Nullable<Integer>,
        IsSender -> Nullable<Integer>,
        CreateTime -> Nullable<Integer>,
        Sequence -> Nullable<Integer>,
        StatusEx -> Nullable<Integer>,
        FlagEx -> Nullable<Integer>,
        Status -> Nullable<Integer>,
        MsgServerSeq -> Nullable<Integer>,
        MsgSequence -> Nullable<Integer>,
        StrTalker -> Nullable<Text>,
        StrContent -> Nullable<Text>,
        DisplayContent -> Nullable<Text>,
        Reserved0 -> Nullable<Integer>,
        Reserved1 -> Nullable<Integer>,
        Reserved2 -> Nullable<Integer>,
        Reserved3 -> Nullable<Integer>,
        Reserved4 -> Nullable<Text>,
        Reserved5 -> Nullable<Text>,
        Reserved6 -> Nullable<Text>,
        CompressContent -> Nullable<Binary>,
        BytesExtra -> Nullable<Binary>,
        BytesTrans -> Nullable<Binary>,
    }
}

diesel::table! {
    MSGTrans (msgLocalId) {
        msgLocalId -> Nullable<Integer>,
        talkerId -> Nullable<Integer>,
    }
}

diesel::table! {
    Name2ID (UsrName) {
        UsrName -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    DBInfo,
    MSG,
    MSGTrans,
    Name2ID,
);