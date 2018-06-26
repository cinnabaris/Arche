class CreateForumTopics < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_topics do |t|
      t.references :category, null: false
      t.references :user, null: false
      t.string :title, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.datetime :deleted_at
      t.timestamps
    end
    add_index :forum_topics, :title

    create_table :forum_topics_badges do |t|
      t.references :badage, null: false
      t.references :topic, null: false
    end
    add_index :forum_topics_badges, %i[badage_id topic_id], unique: true
  end
end
