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

    cy.signIn(email, 'my-password');

    cy.contains('Hello, Emily Thompson!');

    cy.visit('http://localhost:3000/');

    cy.contains('Hello, Emily Thompson!');
  })

  it('redirects with message when password is wrong', () => {
    cy.visit('http://localhost:3000/');

    const email = faker.internet.email();
    cy.signUp({ email });

    cy.contains('Hello, Emily Thompson!');

    cy.contains('Sign Out').click();

    cy.contains('Hello, stranger!');

    cy.signIn(email, 'wrong-password');

    cy.contains('Incorrect email and/or password');
  })

  it('redirects with message when the user does not exist', () => {
    cy.visit('http://localhost:3000/');

    cy.signIn('wrong-email@example.com', 'password');

    cy.contains('Incorrect email and/or password');
  })
})
