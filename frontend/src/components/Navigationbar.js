import React, { useState, useEffect } from 'react';
import { Navbar, Nav, NavDropdown } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';

const Navigationbar = () => {
	const [admin, setAdmin] = useState(false);

	useEffect(() => {
		setAdmin(window.location.href.includes('/admin/'));
	}, []);

	return (
		<header>
			<Navbar bg='dark' variant='dark'>
				<LinkContainer to='/'>
					<Navbar.Brand>Viveack Realty Inc.</Navbar.Brand>
				</LinkContainer>
				<Navbar.Toggle aria-controls='basic-navbar-nav' />
				<Navbar.Collapse id='basic-navbar-nav'>
					<Nav className='ml-auto'>
						{admin && (
							<LinkContainer to='/admin/'>
								<Nav.Link>Admin Screen</Nav.Link>
							</LinkContainer>
						)}
						<Nav.Link href='tel:123-456-7890'>555-555-5555</Nav.Link>
						<NavDropdown title='Properties' id='basic-nav-dropdown'>
							<LinkContainer to='/properties'>
								<NavDropdown.Item>Our Properties</NavDropdown.Item>
							</LinkContainer>
							<NavDropdown.Item>Search For A Property</NavDropdown.Item>
							<NavDropdown.Item>Free Home Evaluation</NavDropdown.Item>
							<NavDropdown.Item>Buying A Home</NavDropdown.Item>
							<NavDropdown.Item>NeighbourHoods</NavDropdown.Item>
						</NavDropdown>
						<NavDropdown title='About Us' id='basic-nav-dropdown'>
							<LinkContainer to='/team'>
								<NavDropdown.Item>Meet The Team</NavDropdown.Item>
							</LinkContainer>
							<NavDropdown.Item href='#action/3.2'>
								Marketing Your Home
							</NavDropdown.Item>
							<LinkContainer to='/blog'>
								<NavDropdown.Item href='#action/3.2'>Our Blog</NavDropdown.Item>
							</LinkContainer>
							<NavDropdown.Item href='#action/3.2'>Events</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.2'>Mentoring</NavDropdown.Item>
							<NavDropdown.Divider />
							<LinkContainer to='/contact'>
								<NavDropdown.Item>Contact Us</NavDropdown.Item>
							</LinkContainer>
						</NavDropdown>
					</Nav>
				</Navbar.Collapse>
			</Navbar>
		</header>
	);
};

export default Navigationbar;
