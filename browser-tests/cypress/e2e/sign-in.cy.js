import { faker } from '@faker-js/faker';

describe('Sign In', () => {
  it('creates a new Account, and then signs in with it', () => {
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

    cy.contains('Hello, stranger!');

    cy.contains('Sign In').click();

    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Hello, Emily Thompson!');

    cy.visit('http://localhost:3000/');

    cy.contains('Hello, Emily Thompson!');
  })

  it('redirects to root when password is wrong', () => {
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

    cy.contains('Hello, stranger!');

    cy.contains('Sign In').click();

    cy.get('input[name="email"]').type(email);
    cy.get('input[name="password"]').type('wrong-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Incorrect email and/or password');
  })

  it('redirects to root when the user does not exist', () => {
    cy.visit('http://localhost:3000/');

    cy.contains('Sign In').click();

    cy.get('input[name="email"]').type("wrong-email@example.com");
    cy.get('input[name="password"]').type('password');
    cy.get('input[type="submit"]').click();

    cy.contains('Incorrect email and/or password');
  })
})
