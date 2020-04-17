-- Your SQL goes here
Create Table users (
    id Serial Primary Key,
    username Varchar Not Null,
    profile Varchar(100) Default 'Images/boy.jpg'
)