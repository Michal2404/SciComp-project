# trying some q-learning reinforcement learning here
import numpy as np
import random
import tensorflow as tf
from collections import deque

class DeepQNetwork:
    def __init__(self, state_size, action_size, replay_memory_size=100000, batch_size=64, gamma=0.99, epsilon=1.0, epsilon_decay=0.995, epsilon_min=0.01, learning_rate=0.001):
        self.state_size = state_size
        self.action_size = action_size
        self.replay_memory = deque(maxlen=replay_memory_size)
        self.batch_size = batch_size
        self.gamma = gamma
        self.epsilon = epsilon
        self.epsilon_decay = epsilon_decay
        self.epsilon_min = epsilon_min
        self.learning_rate = learning_rate

        # Build the Q-network
        self.model = self._build_model()

    def _build_model(self):
        # Create a simple neural network for Q-learning
        model = tf.keras.Sequential([
            tf.keras.layers.Dense(24, input_dim=self.state_size, activation='relu'),
            tf.keras.layers.Dense(24, activation='relu'),
            tf.keras.layers.Dense(self.action_size, activation='linear')
        ])
        model.compile(optimizer=tf.keras.optimizers.Adam(learning_rate=self.learning_rate), loss='mse')
        return model

    def remember(self, state, action, reward, next_state, done):
        self.replay_memory.append((state, action, reward, next_state, done))

    def act(self, state):
        if np.random.rand() <= self.epsilon:
            return random.randrange(self.action_size)
        q_values = self.model.predict(state, verbose=0)
        return np.argmax(q_values[0])

    def replay(self):
        if len(self.replay_memory) < self.batch_size:
            return

        minibatch = random.sample(self.replay_memory, self.batch_size)
        for state, action, reward, next_state, done in minibatch:
            target = reward
            if not done:
                target += self.gamma * np.amax(self.model.predict(next_state, verbose=0)[0])
            target_f = self.model.predict(state, verbose=0)
            target_f[0][action] = target
            self.model.fit(state, target_f, epochs=1, verbose=0)

        if self.epsilon > self.epsilon_min:
            self.epsilon *= self.epsilon_decay

# Deep Q-Learning with Experience Replay
def deep_q_learning(env, episodes=1000, max_timesteps=200):
    state_size = env.observation_space.shape[0]
    action_size = env.action_space.n
    agent = DeepQNetwork(state_size, action_size)

    for episode in range(1, episodes + 1):
        state = env.reset()
        state = np.reshape(state, [1, state_size])
        for t in range(1, max_timesteps + 1):
            # Select action
            action = agent.act(state)
            
            # Execute action in the environment
            next_state, reward, done, _ = env.step(action)
            next_state = np.reshape(next_state, [1, state_size])

            # Store experience in replay memory
            agent.remember(state, action, reward, next_state, done)

            # Move to the next state
            state = next_state

            # Replay experiences to train the network
            agent.replay()

            if done:
                print(f"Episode {episode}/{episodes} ended after {t} timesteps with reward {reward}")
                break
