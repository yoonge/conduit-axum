use crate::db::Topic;

pub fn format(topics: Vec<Topic>) -> Result<Vec<Topic>, anyhow::Error> {
    let mut format_topics = vec![];
    topics.iter().for_each(|topic| {
        let topic = topic.clone();
        let content_clip = if topic.content.len() > 200 {
            topic.content[0..=200].to_string() + "..."
        } else {
            topic.content.clone()
        };
        let title_clip = if topic.title.len() > 160 {
            topic.title[0..=160].to_string() + "..."
        } else {
            topic.title.clone()
        };
        let format_topic = Topic {
            content_clip: Some(content_clip),
            title_clip: Some(title_clip),
            update_at_str: Some(format!("{}", topic.update_at.format("%Y-%m-%d %H:%M:%S"))),
            ..topic
        };

        format_topics.push(format_topic);
    });

    Ok(format_topics)
}
