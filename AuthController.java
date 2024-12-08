import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.SignatureAlgorithm;
import org.springframework.web.bind.annotation.*;

import java.util.Date;
import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/api")
public class AuthController {

    // Секретний ключ для генерації токенів
    private static final String SECRET_KEY = "your_secret_key";

    // База даних користувачів (імітація)
    private static final Map<String, String> users = new HashMap<>();

    static {
        users.put("testuser", "password123");
        users.put("admin", "adminpassword");
    }

    @PostMapping("/login")
    public Map<String, String> login(@RequestBody Map<String, String> userData) {
        String username = userData.get("username");
        String password = userData.get("password");

        if (users.containsKey(username) && users.get(username).equals(password)) {
            String token = Jwts.builder()
                    .setSubject(username)
                    .setIssuedAt(new Date())
                    .setExpiration(new Date(System.currentTimeMillis() + 120000)) // Термін дії: 2 хвилини
                    .signWith(SignatureAlgorithm.HS256, SECRET_KEY)
                    .compact();

            Map<String, String> response = new HashMap<>();
            response.put("access_token", token);
            return response;
        }

        throw new RuntimeException("Invalid username or password");
    }
}
