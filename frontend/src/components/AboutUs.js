import React from 'react';
import { Container, Col, Row, Card, Button } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';

const AboutUs = () => {
	return (
		<Container className='py-5'>
			<Card>
				<Card.Img
					src='images/team.jpg'
					style={{
						width: '100%',
						height: '40vh',
						objectFit: 'cover',
					}}
				/>
				<Card.Body>
					<Card.Title>About Us</Card.Title>
					<Card.Text>
						Some quick example text to build on the card title and make up the
						bulk of the card's content.
					</Card.Text>
					<Row>
						<Col>
							<LinkContainer to='/team'>
								<Button block variant='primary'>Meet The Team</Button>
							</LinkContainer>
						</Col>
						<Col>
							<LinkContainer to='/contact'>
								<Button block variant='primary'>Contact Us</Button>
							</LinkContainer>
						</Col>
					</Row>
				</Card.Body>
			</Card>
		</Container>
	);
};

export default AboutUs;
