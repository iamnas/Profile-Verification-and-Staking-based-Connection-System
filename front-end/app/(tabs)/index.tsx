import { StyleSheet } from 'react-native';
import { ProfileForm } from '../../screens/ProfileForm';
import { View } from 'react-native';

export default function TabOneScreen() {
  return (
    <View style={styles.container}>
      <ProfileForm />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
  },
});
