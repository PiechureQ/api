table! {
    places (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
