class CreateCaringMembers < ActiveRecord::Migration[5.2]
  def change
    create_table :caring_members do |t|
      t.string :nick_name, null: false, limit: 255
      t.string :real_name, null: false, limit: 255
      t.string :phone, limit: 255
      t.string :email, limit: 255
      t.string :address, limit: 255
      t.string :line, limit: 255
      t.string :wechat, limit: 255
      t.string :weibo, limit: 255
      t.string :facebook, limit: 255
      t.timestamps
    end
    add_index :caring_members, :nick_name, unique: true
    add_index :caring_members, :real_name
  end
end
