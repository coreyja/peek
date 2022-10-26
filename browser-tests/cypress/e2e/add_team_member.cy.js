describe('Adding a Team Member', () => {
    it('remembers the team member when added', () => {
        cy.visit('http://localhost:3000/');

        cy.contains('Add Team Member').click();

        cy.contains('Sign In')

        cy.signUp();

        cy.contains('Hello, Emily Thompson!');

        cy.contains('Add Team Member').click();

        cy.contains('New Team Member');

        cy.get('input[name="name"]').type('John Smith');
        cy.get('input[name="zipCode"]').type('04009');
        cy.get('input[name="title"]').type('Software Engineer');
        cy.get('input[name="interests"]').type('Helps in the local band with his cousin');

        cy.get('input[type="submit"]').click();

        cy.contains('Hello, Emily Thompson!');

        cy.contains('John Smith');
        cy.contains('Bridgeton, ME');
    })
})
