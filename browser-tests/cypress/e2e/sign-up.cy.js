import { faker } from '@faker-js/faker';

describe('Sign Up', () => {
  it('creates a new Account', () => {
    cy.visit('http://localhost:3000/');

    cy.contains('Sign Up').click();

    cy.get('input[name="name"]').type('Emily Thompson');

    const email = faker.internet.email();
    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[name="passwordConfirmation"]').type('my-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Hello, Emily Thompson!');

    cy.contains('Sign Out').click();

    cy.contains('Sign In').click();

    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Hello, Emily Thompson!');

    cy.visit('http://localhost:3000/');

    cy.contains('Hello, Emily Thompson!');
  })

  it('errors when the email is already taken', () => {
    cy.visit('http://localhost:3000/');

    cy.contains('Sign Up').click();

    cy.get('input[name="name"]').type('Emily Thompson');

    const email = faker.internet.email();
    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[name="passwordConfirmation"]').type('my-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Hello, Emily Thompson!');

    cy.contains('Sign Out').click();

    cy.contains('Sign Up').click();

    cy.get('input[name="name"]').type('Emily Thompson');

    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[name="passwordConfirmation"]').type('my-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Email has already been taken');
  })

  it('errors when the passwords dont match', () => {
    cy.visit('http://localhost:3000/');

    cy.contains('Sign Up').click();

    cy.get('input[name="name"]').type('Emily Thompson');

    const email = faker.internet.email();
    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[name="passwordConfirmation"]').type('my-password-typo');
    cy.get('input[type="submit"]').click();

    cy.contains('Passwords do not match');
  })
})
