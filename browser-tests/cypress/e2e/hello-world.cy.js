describe('/', () => {
  it('contains hello world', () => {
    cy.visit('http://localhost:3000/');

    cy.get('h1').contains('Hello, World!');
  })
})
