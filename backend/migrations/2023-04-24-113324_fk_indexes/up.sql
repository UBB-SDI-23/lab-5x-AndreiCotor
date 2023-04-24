CREATE INDEX idx_fk_submission_pid ON submissions(problem_id);
create index idx_fk_submission_uid on submissions(user_id);
create index idx_fk_participates_uid on participates(uid);
create index idx_fk_participates_cid on participates(cid);