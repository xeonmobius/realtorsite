import React, { useState } from 'react';
import { Form, Button, Container } from 'react-bootstrap';
import axios from 'axios';

const LoginScreen = ({history}) => {
	const [email, setEmail] = useState('');
	const [password, setPassword] = useState('');
    const [error, setError] = useState('');

	const submitHandler = async (e) => {
		e.preventDefault();
		const user = {
			email: email,
			password: password,
		};

        const {data} = await axios.post('/admin/login', user)

        if (data === true) {
            history.push("/admin")
        } else {
            setError(true)
        }
	};

	return (
		<Container className='py-3'>
			<Form onSubmit={submitHandler}>
				<Form.Group controlId='formBasicEmail'>
					<Form.Label>Email address</Form.Label>
					<Form.Control
						type='email'
						placeholder='Enter email'
						value={email}
						onChange={(e) => setEmail(e.target.value)}
					/>
					<Form.Text className='text-muted'>
						We'll never share your email with anyone else.
					</Form.Text>
				</Form.Group>

				<Form.Group controlId='formBasicPassword'>
					<Form.Label>Password</Form.Label>
					<Form.Control
						type='password'
						placeholder='Enter Password'
						value={password}
						onChange={(e) => setPassword(e.target.value)}
					/>
				</Form.Group>
				<Button variant='primary' type='submit'>
					Submit
				</Button>
			</Form>
            {error && <h5 className="py-2">Wrong password or username</h5>}
		</Container>
	);
};

export default LoginScreen;
