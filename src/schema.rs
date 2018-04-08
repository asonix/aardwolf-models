table! {
    base_actors (id) {
        id -> Int4,
        display_name -> Varchar,
        profile_url -> Varchar,
        inbox_url -> Varchar,
        outbox_url -> Varchar,
        local_user -> Nullable<Int4>,
        follow_policy -> Varchar,
        original_json -> Jsonb,
    }
}

table! {
    base_posts (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        media_type -> Varchar,
        posted_by -> Nullable<Int4>,
        icon -> Nullable<Int4>,
        visibility -> Varchar,
        original_json -> Jsonb,
    }
}

table! {
    comments (id) {
        id -> Int4,
        conversation -> Int4,
        parent -> Int4,
        post -> Int4,
    }
}

table! {
    emails (id) {
        id -> Int4,
        email -> Varchar,
        user_id -> Int4,
        verified -> Bool,
        verification_token -> Nullable<Varchar>,
    }
}

table! {
    event_notifications (id) {
        id -> Int4,
        event_id -> Int4,
        timer_id -> Int4,
    }
}

table! {
    events (id) {
        id -> Int4,
        owner -> Int4,
        start_date -> Int4,
        end_date -> Int4,
        timezone -> Varchar,
        title -> Text,
        description -> Text,
    }
}

table! {
    files (id) {
        id -> Int4,
        file_path -> Varchar,
    }
}

table! {
    followers (id) {
        id -> Int4,
        follower -> Int4,
        follows -> Int4,
    }
}

table! {
    follow_requests (id) {
        id -> Int4,
        follower -> Int4,
        requested_follow -> Int4,
    }
}

table! {
    images (id) {
        id -> Int4,
        width -> Int4,
        height -> Int4,
        file_id -> Int4,
    }
}

table! {
    links (id) {
        id -> Int4,
        href -> Varchar,
        href_lang -> Varchar,
        height -> Nullable<Int4>,
        width -> Nullable<Int4>,
        preview -> Nullable<Text>,
        base_post -> Int4,
    }
}

table! {
    local_auth (id) {
        id -> Int4,
        password -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    media_posts (id) {
        id -> Int4,
        file_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    permissions (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    personas (id) {
        id -> Int4,
        default_visibility -> Varchar,
        is_searchable -> Bool,
        avatar -> Nullable<Int4>,
        shortname -> Varchar,
        base_actor -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        content -> Text,
        source -> Nullable<Text>,
        base_post -> Int4,
    }
}

table! {
    reactions (id) {
        id -> Int4,
        reaction_type -> Varchar,
        comment_id -> Int4,
    }
}

table! {
    role_permissions (id) {
        id -> Int4,
        role_id -> Int4,
        permission_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    timers (id) {
        id -> Int4,
        fire_time -> Timestamptz,
    }
}

table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamptz,
        primary_email -> Nullable<Int4>,
    }
}

joinable!(base_actors -> users (local_user));
joinable!(base_posts -> base_actors (posted_by));
joinable!(base_posts -> images (icon));
joinable!(event_notifications -> events (event_id));
joinable!(event_notifications -> timers (timer_id));
joinable!(events -> personas (owner));
joinable!(images -> files (file_id));
joinable!(links -> base_posts (base_post));
joinable!(local_auth -> users (user_id));
joinable!(media_posts -> files (file_id));
joinable!(media_posts -> posts (post_id));
joinable!(personas -> base_actors (base_actor));
joinable!(personas -> images (avatar));
joinable!(posts -> base_posts (base_post));
joinable!(reactions -> comments (comment_id));
joinable!(role_permissions -> permissions (permission_id));
joinable!(role_permissions -> roles (role_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    base_actors,
    base_posts,
    comments,
    emails,
    event_notifications,
    events,
    files,
    followers,
    follow_requests,
    images,
    links,
    local_auth,
    media_posts,
    permissions,
    personas,
    posts,
    reactions,
    role_permissions,
    roles,
    timers,
    user_roles,
    users,
);
