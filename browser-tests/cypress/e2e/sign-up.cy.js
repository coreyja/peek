import { faker } from '@faker-js/faker';

describe('Sign Up', () => {
  it('creates a new Account', () => {
    cy.signUp();

    cy.isLoggedIn();
  })

  it('errors when the email is already taken', () => {
    const email = faker.internet.email();
    cy.signUp({ email });

    cy.isLoggedIn();

    cy.signOut();

    cy.signUp({ email });

    cy.contains('Email has already been taken');
  })

  it('errors when the passwords dont match', () => {
    cy.signUp({ password: 'my-password', passwordConfirmation: 'my-password-typo' });

    cy.contains('Passwords do not match');
  })
})
