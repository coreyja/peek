describe('Sign Up', () => {
  it('creates a new Account', () => {
    cy.visit('http://localhost:3000/');

    cy.contains('Sign Up').click();

    cy.get('input[name="email"]').type('emily@example.com');
    cy.get('input[name="name"]').type('Emily Thompson');
    cy.get('input[name="password"]').type('my-password');
    cy.get('input[name="passwordConfirmation"]').type('my-password');
    cy.get('input[type="submit"]').click();

    cy.contains('Hello, Emily Thompson!');
  })
})
