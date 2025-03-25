-- Your SQL goes here
ALTER TABLE Properties
ADD COLUMN agent_id INT REFERENCES Agents(id);
