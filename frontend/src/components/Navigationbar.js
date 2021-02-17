import React from 'react';
import { Navbar, Nav, NavDropdown } from 'react-bootstrap';

const Navigationbar = () => {
	return (
		<header>
			<Navbar bg='dark' variant="dark">
				<Navbar.Brand href='#home'>Viveack Realty Inc.</Navbar.Brand>
				<Navbar.Toggle aria-controls='basic-navbar-nav' />
				<Navbar.Collapse id='basic-navbar-nav'>
					<Nav className='ml-auto'>
					<Nav.Link href="tel:123-456-7890">555-555-5555</Nav.Link>
						<NavDropdown title='Properties' id='basic-nav-dropdown'>
							<NavDropdown.Item href='#action/3.1'>
								Our Properties
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.3'>
								Search For A Property
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.4'>
								Free Home Evaluation
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.4'>
								Buying A Home
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.4'>
								NeighbourHoods
							</NavDropdown.Item>
						</NavDropdown>
						<NavDropdown title='About Us' id='basic-nav-dropdown'>
							<NavDropdown.Item href='#action/3.1'>
								Meet The Team
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.2'>
								Marketing Your Home
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.2'>
								Our Blog
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.2'>
								Events
							</NavDropdown.Item>
							<NavDropdown.Item href='#action/3.2'>
								Mentoring
							</NavDropdown.Item>
							<NavDropdown.Divider />
							<NavDropdown.Item href='#action/3.2'>
								Contact Us
							</NavDropdown.Item>
						</NavDropdown>
					</Nav>
				</Navbar.Collapse>
			</Navbar>
		</header>
	);
};

export default Navigationbar;
