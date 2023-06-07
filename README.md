# About
This is a web application that is supposed to clone some of the functionalities of https://infoarena.ro/.
It's backend is written in Rust using the Actix web framework. Cargo is used as package manager. The frontend is written in TypeScript using the React framework. As database Postgres was used, and as an ORM and database migration manager Diesel. 
## More on the tech stack choices
The advantages of this tech stack are:

 - Rust and Actix web are "blazing fast". Also Rust's Tokio async threads have the lowest overhead of any high level language.
 - Rust is type-safe, and through its restrictions makes sure the web server is robust (there is no null, and no throwing of errors, enums are used instead).
 - Rust has a great community and documentation.
 - React has an active community and reduces boilerplate code a lot

Some tradeoffs might be:
	

 - Actix and Diesel are young libraries: they miss some features, and they still haven't been adopted on a large number of projects.

## Getting the app to run

In the next few lines the method of starting the app in production will be presented. In order to do this, first you need a virtual machine with Docker and Postgres installed and a Netlify account.

### Deploying the backend

 1. Clone the repository inside your virtual machine
 2. Set your virtual machine's firewall rules such that it can accept connections on ports `80` and `443`
 3. Register a domain name. Obtain https certificates for this domain. Place the certificate files (`fullchain.pem`, `privkey,pem`, `ssldhparams.pem`) inside the `backend/nginx` folder.
 4.  Run docker-compose using the following command: `docker-compose -f backend/docker-compose-deploy.yml up`. This will create the database (if it doesn't exist), and will run the migrations
 5. The server should be running now. If you wish to populate the database, locate the folder `data-generator`. Here you can run the `generator.py` file, using `python3 generator.py`. When it finishes, it will generate a file called `script.sql`. Using the `psql -q -f script.sql` you can run the inserts.
 6. Test the API using Postman.

### Deploying the frontend
1. Set the backend and web-socket links in the `config.ts` file.
2. Run the Cypress tests.
3. Deploy the project on Netlify.
 

 

# Lab 5x assignment

To make things easier with deployments, you will be using this repository for a while. Check this link weekly for new requirements: https://github.com/UBB-SDI-23/Lab-5x-assignments/blob/main/README.md 

Commit and push everything to this repository.
