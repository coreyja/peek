describe('/', () => {
  it('contains hello world', () => {
    cy.visit('http://localhost:3000/');

    cy.contains('Hello, World!');
  })
})
