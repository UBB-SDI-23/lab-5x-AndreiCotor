describe('Problemarena E2E', () => {
    beforeEach(() => {
        cy.viewport(1722, 824)
        cy.visit('http://localhost:3000/')
    })

    it('login', () => {
        cy.contains('Login').click()
        cy.url().should('include', '/login')

        cy.get('input[name=username]').type("non-existing-user")
        cy.get('input[name=password]').type("password")
        cy.contains("Login").get("#submit").click()
        cy.contains('Invalid username!').should('exist')

        cy.get('input[name=username]').clear().type("andrei")
        cy.get('input[name=password]').clear().type("password")
        cy.contains("Login").get("#submit").click()
        cy.contains('Invalid password!').should('exist')

        cy.get('input[name=username]').clear().type("andrei")
        cy.get('input[name=password]').clear().type("1234567890")
        cy.contains("Login").get("#submit").click()

        cy.url().should('not.include', '/login')
        cy.contains('Welcome, andrei!').should('exist')

        cy.contains('Logout').click()
        cy.contains('Welcome, andrei!').should('not.exist')
        cy.contains('Login').should('exist')
    })

    it('show problems filter', () => {
        cy.get('#all-problems').click()

        cy.get('tr').should('have.length', 4)

        cy.get('input[class=input]').type(1)
        cy.get('tr').should('have.length', 3)


        cy.get('input[class=input]').type("{backspace} {rightArrow} 2")
        cy.get('tr').should('have.length', 2)

        cy.get('input[class=input]').type("{backspace} {rightArrow} 3")
        cy.get('tr').should('have.length', 1)
        cy.contains("No data to show.").should('exist')
    })

    it('show problem edit only for user' ,() => {
        cy.get('#all-problems').click()
        cy.get(".edit").should('not.exist')

        cy.contains('Login').click()
        cy.get('input[name=username]').clear().type("andrei")
        cy.get('input[name=password]').clear().type("1234567890")
        cy.contains("Login").get("#submit").click()

        cy.get('#all-problems').click()
        cy.get(".edit").should('exist')
    })
})