import axios from 'axios';

export const getVideoList = async () => {
  const response = await axios.get('http://localhost:8081/video_list');
  return response.data;
}

