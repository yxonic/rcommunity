
#[derive(UserType, WithData)]
#[with_data(UserData)]
struct User(String);
struct UserData {
    name: String,
}

#[derive(ItemType, WithData)]
#[with_data(PostData)]
struct Post(String);
struct PostData {
    content: String,
}

#[derive(ReactionType, Numerical, Enumerable, Once)]
#[as_numerical(i32)]
enum Vote {
    Upvote = 1,
    Downvote = -1,
}

#[derive(ReactionType, Unique)]
struct Tag(String);

#[derive(ReactionType, ItemType, WithData)]
#[with_data(CommentData)]
struct Comment(String);
struct CommentData {
    content: String,
}

#[derive(ItemType, WithData)]
#[with_data(CommentData)]
struct Item(String);
struct ItemData {
    name: String,
}

#[derive(ReactionType, Numerical, Enumerable, Once)]
#[as_numerical(i32)]
enum Rate {
    R5 = 5,
    R4 = 4,
    R3 = 3,
    R2 = 2,
    R1 = 1,
}

#[derive(ReactionType, Numerical, Enumerable, ItemType)]
#[as_numerical(i32)]
struct Review(Rate, String);

community!(
    struct Community,
    Vote: [User] => [Post, Comment, Post::Tag, Review],
    Tag: [User] => [Post],
    Comment: [User] => [Post, Comment],
    Review: [User] => [Item],
);

fn main() {
    let community = Community::new();

    let uid = "1000";
    let pid = "2001";

    community
        .user()
        .get(uid)
        .post()
        .get(pid)
        .vote()
        .add(Vote::Upvote);

    community.user().get(uid).post().get(pid).vote().get(); // -> Option<Vote>

    community.post().get(pid).vote().received().sum();

    community.post().get(pid).comment().received().list(
        PostCommentReceivedListOption::builder()
            .sort_by(community.comment().sort_by().vote().received().sum())
            .build(),
    );

    community.post().get(pid).tag().received().list(
        PostTagReceivedListOption::builder()
            .sort_by(
                community
                    .post()
                    .get(pid)
                    .tag()
                    .received()
                    .sort_by()
                    .vote()
                    .received()
                    .sum(),
            )
            .build(),
    );
}
