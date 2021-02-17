import React from 'react';
import { Container, Col, Row, Card, Button } from 'react-bootstrap';


const MainHomeWorth = () => {
	return (
		<Container className='py-5'>
			<Row>
				<Col>
					<Card>
						<Card.Img src='images/dollars.jpg' style={{width: '100%', height: '30vh', objectFit: 'cover'}} />
						<Card.Body>
							<Card.Title>What is my home worth?</Card.Title>
							<Card.Text>
								Some quick example text to build on the card title and make up
								the bulk of the card's content.
							</Card.Text>
							<Button variant='primary'>Evaluate My Home</Button>
						</Card.Body>
					</Card>
				</Col>
				<Col>
					<Card>
						<Card.Img src='images/buyhome.jpg' style={{width: '100%', height: '30vh', objectFit: 'cover'}} />
						<Card.Body>
							<Card.Title>Buying A Home</Card.Title>
							<Card.Text>
								Some quick example text to build on the card title and make up
								the bulk of the card's content.
							</Card.Text>
							<Button variant='primary'>Help Me Find My Dream Home</Button>
						</Card.Body>
					</Card>
				</Col>
			</Row>
		</Container>
	);
};

export default MainHomeWorth;
