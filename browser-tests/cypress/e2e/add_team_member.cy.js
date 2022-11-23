import { faker } from '@faker-js/faker';

describe('Adding a Team Member', () => {
    it('remembers the team member when added', () => {
        cy.visit('http://localhost:3000/');

        cy.contains('Sign In')

        const email = faker.internet.email();
        const password = 'my-password';
        cy.signUp({ email, password });


        cy.get('[data-testid="footer"]').contains('Add').click();

        cy.get('input[name="name"]').type('John Smith');
        cy.get('input[name="zipCode"]').type('04009');
        cy.get('input[name="title"]').type('Software Engineer');
        cy.get('input[name="interests"]').type('Helps in the local band with his cousin');

        cy.get('input[type="submit"]').click();

        cy.contains('Welcome to Peek!');

        cy.contains('John Smith');
        cy.contains('04009');

        cy.signOut();

        cy.signIn(email, password);

        cy.contains('Welcome to Peek!');

        cy.contains('John Smith');
        cy.contains('04009');
    })

    it('allows Team Member to be added without a title or interest', () => {
        cy.visit('http://localhost:3000/');

        cy.contains('Sign In')

        const email = faker.internet.email();
        const password = 'my-password';
        cy.signUp({ email, password });


        cy.get('[data-testid="footer"]').contains('Add').click();

        cy.get('input[name="name"]').type('John Smith');
        cy.get('input[name="zipCode"]').type('04009');

        cy.get('input[type="submit"]').click();

        cy.contains('Welcome to Peek!');

        cy.contains('John Smith');
        cy.contains('04009');
    });
})
