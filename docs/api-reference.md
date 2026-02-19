## API Endpoints

### Root Endpoint

**`GET /`**

Returns a simple "Hello, World!" message.

---

### Health Check

**`GET /health`**

Checks the API's health and returns version information.

**Response:**
```json
{
  "status": "Server healthy",
  "version": "0.1.0"
}
```

---

### Word Search

**`GET /search?q=<query>`**

Search for words in the FST dictionary with fuzzy matching (Levenshtein distance: 1). Returns top 10 results prioritized by:
1. Exact matches
2. Higher scores
3. Alphabetical order

**Parameters:**
- `q` (required): The search query string

**Example Request:**
```bash
curl "http://127.0.0.1:8080/search?q=hello"
```

**Example Response:**
```json
[
  {
    "found": "hello",
    "score": "12345",
    "exist": true
  },
  {
    "found": "hallo",
    "score": "11000",
    "exist": true
  }
]
```

**Note:** Returns an empty array `[]` if no matches are found.