import React, { useState } from 'react';
import { Container, Form, Button } from 'react-bootstrap';
import axios from 'axios';

const ContactUsScreen = () => {
	const [name, setName] = useState('');
	const [email, setEmail] = useState('');
	const [phone, setPhone] = useState('');
	const [message, setMessage] = useState('');

	const submitHandler = async (e) => {
		
		e.preventDefault();

		const contact = {
			name: name,
			email: email,
			phone: phone,
			message: message,
		};

		const { data } = await axios.post('/contactus', contact);

		if (data === true) {
			alert('Message recieved!');
			setName('');
			setEmail('');
			setPhone('');
			setMessage('');
		}
	};

	return (
		<Container className='py-3'>
			<h1>Contact Us</h1>
			<Form onSubmit={submitHandler}>
				<Form.Group>
					<Form.Label>Full Name</Form.Label>
					<Form.Control
						type='text'
						placeholder='Enter your name'
						size='lg'
						value={name}
						onChange={(e) => setName(e.target.value)}
					/>
					<Form.Text className='text-muted'>
						Please enter your preferred full name to address you.
					</Form.Text>
				</Form.Group>
				<Form.Group>
					<Form.Label>Email Address</Form.Label>
					<Form.Control
						type='email'
						placeholder='Enter email'
						size='lg'
						value={email}
						onChange={(e) => setEmail(e.target.value)}
					/>
					<Form.Text className='text-muted'>
						We'll never share your email with anyone else.
					</Form.Text>
				</Form.Group>
				<Form.Group>
					<Form.Label>Phone Number</Form.Label>
					<Form.Control
						type='tel'
						placeholder='555-555-5555'
						pattern='[0-9]{3}-[0-9]{3}-[0-9]{4}'
						size='lg'
						value={phone}
						onChange={(e) => setPhone(e.target.value)}
					/>
					<Form.Text className='text-muted'>
						Please enter your phone number.
					</Form.Text>
				</Form.Group>
				<Form.Group>
					<Form.Label>Message</Form.Label>
					<Form.Control
						type='text'
						placeholder=''
						size='lg'
						value={message}
						onChange={(e) => setMessage(e.target.value)}
					/>
					<Form.Text className='text-muted'>
						Please send us a short message on any houses you liked or any
						request you may have.
					</Form.Text>
				</Form.Group>
				<Button variant='primary' type='submit' block size='lg'>
					<h1 style={{ color: 'white' }}>Submit</h1>
				</Button>
			</Form>
		</Container>
	);
};

export default ContactUsScreen;
