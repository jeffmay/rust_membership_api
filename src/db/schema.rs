table! {
    attendees (id) {
        id -> Int4,
        meeting_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    candidates (id) {
        id -> Int4,
        election_id -> Int4,
        name -> Text,
        image_url -> Nullable<Text>,
    }
}

table! {
    committees (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    elections (id) {
        id -> Int4,
        name -> Text,
        status -> Text,
        number_winners -> Int4,
        voting_begins -> Nullable<Timestamp>,
        voting_ends -> Nullable<Timestamp>,
    }
}

table! {
    eligible_voters (id) {
        id -> Int4,
        user_id -> Int4,
        voted -> Bool,
        election_id -> Int4,
    }
}

table! {
    emails (id) {
        id -> Int4,
        external_id -> Text,
        email_address -> Text,
        committee_id -> Nullable<Int4>,
    }
}

table! {
    forwarding_addresses (id) {
        id -> Int4,
        forward_to -> Text,
        incoming_email_id -> Int4,
    }
}

table! {
    meetings (id) {
        id -> Int4,
        short_id -> Nullable<Int4>,
        name -> Text,
        committee_id -> Nullable<Int4>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
    }
}

table! {
    rankings (id) {
        id -> Int4,
        candidate_id -> Int4,
        vote_id -> Int4,
        ranked -> Int4,
    }
}

table! {
    roles (id) {
        id -> Int4,
        committee_id -> Nullable<Int4>,
        user_id -> Int4,
        role -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email_address -> Nullable<Text>,
        normalized_email -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        biography -> Nullable<Text>,
    }
}

table! {
    votes (id) {
        id -> Int4,
        election_id -> Int4,
        vote_key -> Int4,
    }
}

joinable!(attendees -> meetings (meeting_id));
joinable!(attendees -> users (user_id));
joinable!(candidates -> elections (election_id));
joinable!(eligible_voters -> elections (election_id));
joinable!(eligible_voters -> users (user_id));
joinable!(emails -> committees (committee_id));
joinable!(forwarding_addresses -> emails (incoming_email_id));
joinable!(meetings -> committees (committee_id));
joinable!(rankings -> candidates (candidate_id));
joinable!(rankings -> votes (vote_id));
joinable!(roles -> committees (committee_id));
joinable!(roles -> users (user_id));
joinable!(votes -> elections (election_id));

allow_tables_to_appear_in_same_query!(
    attendees,
    candidates,
    committees,
    elections,
    eligible_voters,
    emails,
    forwarding_addresses,
    meetings,
    rankings,
    roles,
    users,
    votes,
);
