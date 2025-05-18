CREATE TABLE IF NOT EXISTS disciplines (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS memberships (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    price FLOAT NOT NULL,
    discipline_id INT NOT NULL,
    total_classes INT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    duration_days INT DEFAULT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL,
    CONSTRAINT fk_membership_discipline FOREIGN KEY (discipline_id) REFERENCES disciplines(id)
        ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS client_memberships (
    id INT AUTO_INCREMENT PRIMARY KEY,
    client_id INT NOT NULL,
    membership_id INT NOT NULL,
    purchased_at DATETIME NOT NULL,
    remaining_classes INT DEFAULT NULL,
    expires_at DATETIME DEFAULT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL,
    CONSTRAINT fk_client_membership_client FOREIGN KEY (client_id) REFERENCES clients(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_client_membership_membership FOREIGN KEY (membership_id) REFERENCES memberships(id)
        ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS class_attendance (
    id INT AUTO_INCREMENT PRIMARY KEY,
    client_membership_id INT NOT NULL,
    attended_at DATETIME NOT NULL,
    CONSTRAINT fk_class_attendance FOREIGN KEY (client_membership_id) REFERENCES client_memberships(id)
        ON DELETE CASCADE
) ENGINE=InnoDB;