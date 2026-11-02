import unittest
from fastapi.testclient import TestClient

from anime_scout.ui.app import create_app


class TestUiEndpoints(unittest.TestCase):
    def setUp(self):
        self.app = create_app()
        self.client = TestClient(self.app)

    def test_index_page_loads_kusaira(self):
        res = self.client.get("/")
        self.assertEqual(res.status_code, 200)
        self.assertIn("Kusaira", res.text)
        self.assertIn("PTS", res.text)

    def test_search_api_mock(self):
        res = self.client.get("/api/search?q=Mushoku")
        self.assertEqual(res.status_code, 200)
        data = res.json()
        self.assertIn("results", data)
        self.assertIsInstance(data["results"], list)

    def test_player_status_endpoint(self):
        res = self.client.get("/api/player/status")
        self.assertEqual(res.status_code, 200)
        data = res.json()
        self.assertIn("state", data)
        self.assertIn("time", data)


if __name__ == "__main__":
    unittest.main()
