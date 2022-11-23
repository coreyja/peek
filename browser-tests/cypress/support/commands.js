import { faker } from '@faker-js/faker';

Cypress.Commands.add('signIn', (email, password) => {
    password = password || 'my-password';
    cy.visit('http://localhost:3000/');

    cy.contains('Sign In').click();

    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type(password);
    cy.get('input[type="submit"]').click();
});

Cypress.Commands.add('signUp', ({ email, password, passwordConfirmation } = {}) => {
    password = password || 'my-password';
    passwordConfirmation = passwordConfirmation || password;
    email = email || faker.internet.email();

    cy.visit('http://localhost:3000/');

    cy.contains('Sign Up').click();

    cy.get('input[name="name"]').type('Emily Thompson');

    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type(password);
    cy.get('input[name="passwordConfirmation"]').type(passwordConfirmation);
    cy.get('input[type="submit"]').click();
});

Cypress.Commands.add('signOut', () => {
    cy.contains('Profile').click();
    cy.contains('Sign Out').click();
});

Cypress.Commands.add('isLoggedIn', () => {
    cy.contains('Welcome to Peek!');
});

Cypress.Commands.add('isLoggedOut', () => {
    cy.contains('Taking a peek at local weather and news, keeps you connected with your long distance coworkers.');
});

