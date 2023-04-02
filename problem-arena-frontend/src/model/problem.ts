export default class Problem {
    id: number
    name: string
    author: string
    contest: string
    statement: string
    rating: number

    constructor(id: number, name: string, author: string, contest: string, statement: string, rating: number) {
        this.id = id;
        this.name = name;
        this.author = author;
        this.contest = contest;
        this.statement = statement;
        this.rating = rating;
    }
}