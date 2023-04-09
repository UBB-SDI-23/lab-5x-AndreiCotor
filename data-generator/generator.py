from faker import Faker


fake = Faker()
Faker.seed(0)
fo = open("script.sql", "w")

BATCH_SIZE = 1000
NR_BATCHES_SIMPLE = 1000
NR_BATCHES_RELATION = 10_000


def sql_prelude():
    fo.write("DELETE FROM PARTICIPATES;\n")
    fo.write("DELETE FROM SUBMISSIONS;\n")
    fo.write("DELETE FROM PROBLEMS;\n")
    fo.write("DELETE FROM CONTEST;\n")
    fo.write("DELETE FROM USERS;\n")


#problem: name, author, contest, statement, rating
def generate_problem(id: int, final):
    name = fake.word()
    author = fake.name()
    contest = fake.word()
    statement = fake.sentence(nb_words=3)
    rating = fake.random_int(1, 5)
    fo.write("(" + str(id) + ", \'" + name + "\', \'" + author + "\', \'" + contest + "\', \'" + statement + "\', " + str(rating) + ")")
    
    if not final:
        fo.write(",")
    

def generate_problem_batch(start_id: int):
    fo.write("INSERT INTO PROBLEMS (id, name, author, contest, statement, rating) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_problem(start_id + nr, False)
        
    generate_problem(start_id + BATCH_SIZE - 1, True)
    fo.write(";\n")
    
    
# contest: name
def generate_contest(id: int, final):
    name = fake.word()
    fo.write("(" + str(id) + ", \'" + name + "\')")
    
    if not final:
        fo.write(",")
    
    
def generate_contest_batch(start_id: int):
    fo.write("INSERT INTO CONTEST (id, name) VALUES ")
    for nr in range(BATCH_SIZE - 1):
        generate_contest(start_id + nr, False)
    generate_contest(start_id + BATCH_SIZE - 1, True)
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
    user_id = fake.random_int(1, 1000000)
    problem_id = fake.random_int(1, 1000000)
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
        id1 = fake.random_int(1, 1000000)
        id2 = fake.random_int(1, 1000000)
        
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
    for i in range(NR_BATCHES_SIMPLE):
        generate_problem_batch(i * BATCH_SIZE + 1)
        generate_contest_batch(i * BATCH_SIZE + 1)
        generate_user_batch(i * BATCH_SIZE + 1)
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
    
    
if __name__ == '__main__':
    main()
    fo.close()
    