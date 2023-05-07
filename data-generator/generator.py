from faker import Faker


fake = Faker()
Faker.seed(0)
fo = open("script.sql", "w")

BATCH_SIZE = 1000
NR_BATCHES_SIMPLE = 1000
NR_BATCHES_RELATION = 10_000
NR_USER_BATCHES = 10

username_list = [fake.unique.user_name() for _ in range(10000)]
uuid_list = [fake.unique.uuid4() for _ in range(10000)]


def sql_prelude():
    fo.write("drop index idx_username;\n")
    fo.write("drop index idx_fk_contest_uid;\n")
    fo.write("drop index idx_fk_submission_uid;\n")
    fo.write("drop index idx_fk_submission_pid;\n")
    fo.write("drop index idx_fk_problem_uid;\n")
    fo.write("drop index idx_fk_participates_uid;\n")
    fo.write("drop index idx_fk_participates_cid;\n")

    fo.write("alter table contest drop constraint contest_uid_fkey;\n")
    fo.write("alter table participates drop constraint participates_uid_fkey;\n")
    fo.write("alter table participates drop constraint participates_cid_fkey;\n")
    fo.write("alter table problems drop constraint problems_uid_fkey;\n")
    fo.write("alter table submissions drop constraint submissions_problem_id_fkey;\n")
    fo.write("alter table submissions drop constraint submissions_user_id_fkey;\n")
    fo.write("alter table usercredentials drop constraint usercredentials_username_key;\n")
    fo.write("alter table usercredentials drop constraint usercredentials_uuid_key;\n")

    fo.write("alter table contest drop constraint contest_pkey;\n")
    fo.write("alter table participates drop constraint participates_pkey;\n")
    fo.write("alter table problems drop constraint problems_pkey;\n")
    fo.write("alter table submissions drop constraint submissions_pkey;\n")
    fo.write("alter table usercredentials drop constraint usercredentials_pkey;\n")
    fo.write("alter table users drop constraint users_pkey;\n")

    fo.write("DELETE FROM usercredentials;\n")
    fo.write("DELETE FROM PARTICIPATES;\n")
    fo.write("DELETE FROM SUBMISSIONS;\n")
    fo.write("DELETE FROM PROBLEMS;\n")
    fo.write("DELETE FROM CONTEST;\n")
    fo.write("DELETE FROM USERS;\n")


def sql_end():
    fo.write("alter table contest add constraint contest_pkey primary key (id);\n")
    fo.write("alter table participates add constraint participates_pkey primary key (uid, cid);\n")
    fo.write("alter table problems add constraint problems_pkey primary key (id);\n")
    fo.write("alter table submissions add constraint submissions_pkey primary key (id);\n")
    fo.write("alter table usercredentials add constraint usercredentials_pkey primary key (id);\n")
    fo.write("alter table users add constraint users_pkey primary key (id);\n")

    fo.write("alter table contest add constraint contest_uid_fkey foreign key (uid) references users(id);\n")
    fo.write("alter table participates add constraint participates_uid_fkey foreign key (uid) references users(id);\n")
    fo.write("alter table participates add constraint participates_cid_fkey foreign key (cid) references contest(id);\n")
    fo.write("alter table problems add constraint problems_uid_fkey foreign key (uid) references users(id);\n")
    fo.write("alter table submissions add constraint submissions_problem_id_fkey foreign key (problem_id) references problems(id);\n")
    fo.write("alter table submissions add constraint submissions_user_id_fkey foreign key (user_id) references users(id);\n")

    fo.write("create index idx_fk_contest_uid on contest(uid);\n")
    fo.write("create index idx_fk_participates_uid on participates(uid);\n")
    fo.write("create index idx_fk_participates_cid on participates(cid);\n")
    fo.write("create index idx_fk_problem_uid on problems(uid);\n")
    fo.write("create index idx_fk_submission_uid on submissions(user_id);\n")
    fo.write("create index idx_fk_submission_pid on submissions(problem_id);\n")
    fo.write("create index idx_username on usercredentials(username);\n")
    fo.write("alter table usercredentials add constraint usercredentials_username_key unique(username);\n")
    fo.write("alter table usercredentials add constraint usercredentials_uuid_key unique(uuid);\n")


def get_userid():
    return fake.random_int(1, BATCH_SIZE * NR_USER_BATCHES)


def get_entity_id():
    return fake.random_int(1, BATCH_SIZE * NR_BATCHES_SIMPLE)


#problem: name, author, contest, statement, rating, uid
def generate_problem(id: int, final):
    name = fake.word()
    author = fake.name()
    contest = fake.word()
    statement = fake.sentence(nb_words=3)
    rating = fake.random_int(1, 5)
    uid = get_userid()
    fo.write("(" + str(id) + ", \'" + name + "\', \'" + author + "\', \'" + contest + "\', \'" + statement + "\', " + str(rating) + ", " + str(uid) + ")")
    
    if not final:
        fo.write(",")
    

def generate_problem_batch(start_id: int):
    fo.write("INSERT INTO PROBLEMS (id, name, author, contest, statement, rating, uid) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_problem(start_id + nr, False)
        
    generate_problem(start_id + BATCH_SIZE - 1, True)
    fo.write(";\n")
    
    
# contest: name, description, uid
def generate_contest(id: int, final):
    name = fake.word()
    description = '\\n'.join(fake.paragraphs(nb=3))
    uid = get_userid()
    fo.write("(" + str(id) + ", \'" + name + "\', \'" + description + "\', " + str(uid) + ")")
    
    if not final:
        fo.write(",")
    
    
def generate_contest_batch(start_id: int):
    fo.write("INSERT INTO CONTEST (id, name, description, uid) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_contest(start_id + nr, False)
    generate_contest(start_id + BATCH_SIZE - 1, True)
    fo.write(";\n")
    

# username, password, confirmed, uuid
def generate_user_credentials(id: int, final):
    username = username_list[id]
    password = "$argon2id$v=19$m=19456,t=2,p=1$YW5kcmVpZWJvc3Nz$0SaZ1FIgsw6lEBAZfDW8aRrVrktwJ/vsF7YzFPF397I"
    confirmed = "true"
    uuid = uuid_list[id]
    
    fo.write("(" + str(id) + ", \'" + username + "\', \'" + password + "\', " + confirmed + ", \'" + uuid + "\')")

    if not final:
        fo.write(",")


def generate_user_credentials_batch(start_id: int):
    fo.write("INSERT INTO USERCREDENTIALS (id, username, password, confirmed, uuid) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_user_credentials(start_id + nr, False)

    generate_user_credentials(start_id + BATCH_SIZE - 1, True)
    fo.write(";\n")


# user: fname, lname, school, bio, teacher
def generate_user(id: int, final):
    fname = fake.first_name()
    lname = fake.last_name()
    school = fake.city() + " Highschool"
    bio = fake.sentence(nb_words=3)
    teacher = fake.name()
    fo.write("(" + str(id) + ", \'" + fname + "\', \'" + lname + "\', \'" + school + "\', \'" + bio + "\', \'" + teacher + "\')")
    
    if not final:
        fo.write(",")
    
    
def generate_user_batch(start_id: int):
    fo.write("INSERT INTO USERS (id, first_name, last_name, school, bio, teacher) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_user(start_id + nr, False)
        
    generate_user(start_id + BATCH_SIZE - 1, True)
    fo.write(";\n")
    

# submission: user_id, problem_id, source_code, score, language
def generate_submission(id: int, final):
    user_id = get_userid()
    problem_id = get_entity_id()
    source_code = fake.sentence(nb_words=3)
    score = fake.random_int(0, 100)
    language = fake.random_element(elements=("C++", "Rust", "Python", "Java", "C#", "Pascal", "Haskell"))
    fo.write("(" + str(id) + ", " + str(user_id) + ", " + str(problem_id) + ", \'" + source_code + "\', " + str(score) + ", \'" + language + "\')")
    
    if not final:
        fo.write(",")
    

def generate_submission_batch(start_id: int):
    fo.write("INSERT INTO SUBMISSIONS (id, user_id, problem_id, source_code, score, language) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_submission(start_id + nr, False)
        
    generate_submission(start_id + BATCH_SIZE - 1, True)
    fo.write(";\n")
    

composed_key = set()
def generate_composed_key():
    while True:
        id1 = get_userid()
        id2 = get_entity_id()
        
        if not ((id1, id2) in composed_key):
            composed_key.add((id1, id2))
            return id1, id2


# participates: uid, cid, score, official
def generate_participation(final):
    uid, cid = generate_composed_key()
    score = fake.random_int(0, 300)
    official = fake.random_element(elements=("true", "false"))
    fo.write("(" + str(uid) + ", " + str(cid) + ", " + str(score) + ", " + official + ")")
    
    if not final:
        fo.write(",")


def generate_participation_batch():
    fo.write("INSERT INTO PARTICIPATES (uid, cid, score, official) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_participation(False)
        
    generate_participation(True)
    fo.write(";\n")


def generate():
    for i in range(NR_USER_BATCHES):
        generate_user_credentials_batch(i * BATCH_SIZE + 1)
        generate_user_batch(i * BATCH_SIZE + 1)

        if i % 25 == 0:
            print(f"{i}/{NR_USER_BATCHES}")

    for i in range(NR_BATCHES_SIMPLE):
        generate_problem_batch(i * BATCH_SIZE + 1)
        generate_contest_batch(i * BATCH_SIZE + 1)

        if i % 25 == 0:
            print(f"{i}/{NR_BATCHES_SIMPLE}")
    
    for i in range(NR_BATCHES_RELATION):
        generate_submission_batch(i * BATCH_SIZE + 1)
        generate_participation_batch()
        if i % 50 == 0:
            print(f"{i}/{NR_BATCHES_RELATION}")


def main():
    sql_prelude()
    generate()
    sql_end()
    

if __name__ == '__main__':
    main()
    fo.close()
    